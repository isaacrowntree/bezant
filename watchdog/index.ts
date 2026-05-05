/**
 * bezant-watchdog
 *
 * Pi-side liveness watchdog for the bezant Docker container. Runs once per
 * minute via systemd timer. Restarts the container when `/health` returns
 * 5xx or unreachable for 5 consecutive probes — the only failure mode that
 * empirically requires a `docker restart bezant` to recover from.
 *
 * IMPORTANT: this watchdog does NOT restart on `bezant-relogin` having
 * disabled itself. The disabled sentinel is the user's "I'm not around to
 * tap a phone push right now" signal; auto-restarting and re-enabling
 * relogin in that case just spams the phone with IB Key pushes during
 * gym/sleep/meeting hours. If the disabled flag is present, the watchdog
 * leaves it alone and the user re-enables manually when they're ready.
 *
 * After a 5xx-triggered restart we DO clear the disabled sentinel — the
 * working assumption is the disable was caused by the same wedged state
 * the restart just fixed, so we want relogin to retry.
 *
 * 2-hour cooldown between restarts prevents loops on persistent issues.
 *
 * Logs go to stdout → systemd journal. Tail with:
 *   journalctl --user -u bezant-watchdog -f
 */
import 'dotenv/config';
import fs from 'node:fs/promises';
import path from 'node:path';
import os from 'node:os';
import { exec } from 'node:child_process';
import { promisify } from 'node:util';

const execAsync = promisify(exec);

// ---------- config ----------

const HEALTH_URL = process.env.BEZANT_HEALTH_URL ?? 'http://localhost:8080/health';
const CONTAINER_NAME = process.env.BEZANT_CONTAINER ?? 'bezant';
const RELOGIN_DISABLED_FILE =
  process.env.BEZANT_RELOGIN_DISABLED_FILE ??
  path.join(os.homedir(), '.local', 'state', 'bezant-relogin', 'disabled');
const RELOGIN_STATE_FILE =
  process.env.BEZANT_RELOGIN_STATE_FILE ??
  path.join(os.homedir(), '.local', 'state', 'bezant-relogin', 'state.json');
const STATE_DIR =
  process.env.BEZANT_WATCHDOG_STATE_DIR ??
  path.join(os.homedir(), '.local', 'state', 'bezant-watchdog');
const STATE_FILE = path.join(STATE_DIR, 'state.json');

const HEALTH_TIMEOUT_MS = 5_000;
const SERVER_ERROR_THRESHOLD = 5; // consecutive 5xx/unreachable probes before restart
const RESTART_COOLDOWN_MS = 2 * 60 * 60 * 1000; // 2 hours minimum between restarts
const POST_RESTART_HEALTH_PROBES = 12; // wait up to 60s for /health to come back
const POST_RESTART_PROBE_INTERVAL_MS = 5_000;

// ---------- types ----------

type HealthState = 'authenticated' | 'not_authenticated' | 'server_error' | 'unreachable';

interface WatchdogState {
  lastHealthState: HealthState | null;
  consecutiveServerErrors: number;
  lastRestartAt: string | null;
  lastRestartReason: string | null;
  totalRestarts: number;
}

const DEFAULT_STATE: WatchdogState = {
  lastHealthState: null,
  consecutiveServerErrors: 0,
  lastRestartAt: null,
  lastRestartReason: null,
  totalRestarts: 0,
};

// ---------- state persistence ----------

async function loadState(): Promise<WatchdogState> {
  try {
    const raw = await fs.readFile(STATE_FILE, 'utf8');
    return { ...DEFAULT_STATE, ...JSON.parse(raw) };
  } catch {
    return { ...DEFAULT_STATE };
  }
}

async function saveState(state: WatchdogState): Promise<void> {
  await fs.mkdir(STATE_DIR, { recursive: true });
  const tmp = STATE_FILE + '.tmp';
  await fs.writeFile(tmp, JSON.stringify(state, null, 2));
  await fs.rename(tmp, STATE_FILE);
}

// ---------- probes ----------

async function probeHealth(): Promise<HealthState> {
  try {
    const res = await fetch(HEALTH_URL, { signal: AbortSignal.timeout(HEALTH_TIMEOUT_MS) });
    if (res.status === 200) {
      const body = (await res.json()) as { authenticated?: boolean };
      return body.authenticated ? 'authenticated' : 'not_authenticated';
    }
    if (res.status === 401) return 'not_authenticated';
    return 'server_error';
  } catch {
    return 'unreachable';
  }
}

async function pathExists(p: string): Promise<boolean> {
  try {
    await fs.access(p);
    return true;
  } catch {
    return false;
  }
}

async function getReloginFailureCount(): Promise<number> {
  try {
    const raw = await fs.readFile(RELOGIN_STATE_FILE, 'utf8');
    const s = JSON.parse(raw) as { consecutiveFailures?: unknown };
    return typeof s.consecutiveFailures === 'number' ? s.consecutiveFailures : 0;
  } catch {
    return 0;
  }
}

// ---------- actions ----------

async function restartContainer(reason: string): Promise<boolean> {
  log(`RESTARTING ${CONTAINER_NAME}: ${reason}`);
  try {
    await execAsync(`docker restart ${CONTAINER_NAME}`, { timeout: 60_000 });
    log(`docker restart returned successfully — waiting for /health to respond`);
  } catch (err) {
    log(`docker restart FAILED: ${(err as Error).message}`);
    return false;
  }
  for (let i = 0; i < POST_RESTART_HEALTH_PROBES; i++) {
    await new Promise((r) => setTimeout(r, POST_RESTART_PROBE_INTERVAL_MS));
    const h = await probeHealth();
    if (h !== 'unreachable') {
      log(`Post-restart /health responsive: ${h}`);
      return true;
    }
  }
  log(`Post-restart /health still unreachable after 60s`);
  return false;
}

async function clearReloginDisabled(): Promise<void> {
  try {
    await fs.unlink(RELOGIN_DISABLED_FILE);
    log('Cleared bezant-relogin disabled sentinel — next 5-min relogin tick will retry');
  } catch {
    /* not present, no-op */
  }
}

// ---------- logging ----------

function log(msg: string): void {
  console.log(`[${new Date().toISOString()}] [watchdog] ${msg}`);
}

// ---------- main ----------

async function main(): Promise<void> {
  const state = await loadState();
  const now = new Date();

  const currentHealth = await probeHealth();
  if (state.lastHealthState !== currentHealth) {
    log(`/health transition: ${state.lastHealthState ?? '<first>'} → ${currentHealth}`);
    state.lastHealthState = currentHealth;
  }

  if (currentHealth === 'server_error' || currentHealth === 'unreachable') {
    state.consecutiveServerErrors += 1;
  } else {
    state.consecutiveServerErrors = 0;
  }

  const sinceLastRestart = state.lastRestartAt
    ? now.getTime() - new Date(state.lastRestartAt).getTime()
    : Infinity;
  const cooldownActive = sinceLastRestart < RESTART_COOLDOWN_MS;

  const reloginDisabled = await pathExists(RELOGIN_DISABLED_FILE);
  const reloginFailures = await getReloginFailureCount();

  let restartReason: string | null = null;

  if (cooldownActive) {
    const cdMin = Math.floor((RESTART_COOLDOWN_MS - sinceLastRestart) / 60_000);
    log(
      `status: health=${currentHealth} relogin_failures=${reloginFailures} relogin_disabled=${reloginDisabled} (cooldown ${cdMin}min remaining)`,
    );
  } else if (state.consecutiveServerErrors >= SERVER_ERROR_THRESHOLD) {
    restartReason = `${state.consecutiveServerErrors} consecutive server_error/unreachable probes`;
  } else {
    log(`status: health=${currentHealth} relogin_failures=${reloginFailures} relogin_disabled=${reloginDisabled} (healthy)`);
  }

  if (restartReason) {
    const ok = await restartContainer(restartReason);
    state.lastRestartAt = now.toISOString();
    state.lastRestartReason = restartReason;
    state.consecutiveServerErrors = 0;
    state.totalRestarts += 1;
    if (ok) {
      await clearReloginDisabled();
    }
    state.lastHealthState = await probeHealth();
  }

  await saveState(state);
}

main().catch((err) => {
  log(`Fatal: ${(err as Error).stack ?? err}`);
  process.exit(1);
});
