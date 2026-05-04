/**
 * bezant-relogin
 *
 * Probes bezant-server's /health endpoint. If the IBKR Client Portal Gateway
 * session has expired, drives a Playwright login at https://localhost:5000.
 *
 * 2FA is handled out-of-band: when the script clicks "Sign In", IBKR pushes
 * an IB Key challenge to the user's phone. The script then polls
 * bezant-server's /health endpoint until it reports authenticated=true, or
 * times out after 2 minutes if the user doesn't tap "Approve".
 *
 * After 3 consecutive failed attempts, writes a `disabled` sentinel and
 * exits silently on subsequent timer ticks. Manual reset:
 *
 *   rm ~/.local/state/bezant-relogin/disabled
 *   systemctl --user start bezant-relogin.service
 *
 * Run via: npx tsx index.ts
 */
import 'dotenv/config';
import fs from 'node:fs/promises';
import path from 'node:path';
import os from 'node:os';
import { chromium, type Browser } from 'playwright';

// ---------- config ----------

const HEALTH_URL = process.env.BEZANT_HEALTH_URL ?? 'http://localhost:8080/health';
const LOGIN_URL = process.env.BEZANT_LOGIN_URL ?? 'https://localhost:5000';
const USERNAME = process.env.IBKR_USERNAME;
const PASSWORD = process.env.IBKR_PASSWORD;
const POST_LOGIN_TIMEOUT_MS = 2 * 60 * 1000; // 2 min for IB Key tap
const HEALTH_POLL_INTERVAL_MS = 5_000;
const MAX_CONSECUTIVE_FAILURES = 3;

const STATE_DIR = process.env.BEZANT_RELOGIN_STATE_DIR
  ?? path.join(os.homedir(), '.local', 'state', 'bezant-relogin');
const STATE_FILE = path.join(STATE_DIR, 'state.json');
const DISABLED_FILE = path.join(STATE_DIR, 'disabled');

// ---------- state ----------

interface KeepaliveState {
  consecutiveFailures: number;
  lastAttemptAt: string | null;
  lastSuccessAt: string | null;
}

const DEFAULT_STATE: KeepaliveState = {
  consecutiveFailures: 0,
  lastAttemptAt: null,
  lastSuccessAt: null,
};

async function loadState(): Promise<KeepaliveState> {
  try {
    const raw = await fs.readFile(STATE_FILE, 'utf8');
    return { ...DEFAULT_STATE, ...JSON.parse(raw) };
  } catch {
    return { ...DEFAULT_STATE };
  }
}

async function saveState(state: KeepaliveState): Promise<void> {
  await fs.mkdir(STATE_DIR, { recursive: true });
  const tmp = STATE_FILE + '.tmp';
  await fs.writeFile(tmp, JSON.stringify(state, null, 2));
  await fs.rename(tmp, STATE_FILE);
}

async function isDisabled(): Promise<boolean> {
  try {
    await fs.access(DISABLED_FILE);
    return true;
  } catch {
    return false;
  }
}

async function setDisabled(): Promise<void> {
  await fs.mkdir(STATE_DIR, { recursive: true });
  await fs.writeFile(DISABLED_FILE, `${new Date().toISOString()}\n`);
}

// ---------- health probe ----------

interface HealthResponse {
  authenticated: boolean;
  connected: boolean;
  competing?: boolean;
  message?: string | null;
}

async function probeHealth(): Promise<HealthResponse | null> {
  try {
    const res = await fetch(HEALTH_URL, { signal: AbortSignal.timeout(10_000) });
    if (!res.ok) return null;
    return (await res.json()) as HealthResponse;
  } catch {
    return null;
  }
}

// ---------- login flow ----------

async function login(browser: Browser): Promise<boolean> {
  const ctx = await browser.newContext({ ignoreHTTPSErrors: true });
  const page = await ctx.newPage();
  try {
    log(`Opening ${LOGIN_URL}`);
    await page.goto(LOGIN_URL, { waitUntil: 'domcontentloaded', timeout: 30_000 });

    // Selectors track CPGateway's stable form. If IBKR redesigns the login
    // page these may need adjusting — confirm against the live page source.
    await page.fill('#user_name, input[name="username"]', USERNAME!);
    await page.fill('#password, input[name="password"]', PASSWORD!);
    await page.click('#submitForm, button[type="submit"], input[type="submit"]');

    log('Submitted credentials. Tap "Approve" on IB Key — waiting up to 2 min...');

    // Use bezant-server's /health as the post-login signal of truth: when
    // CPGateway has minted its internal cookie jar, /health flips to
    // authenticated=true. This sidesteps any need to match a redirect URL.
    const start = Date.now();
    while (Date.now() - start < POST_LOGIN_TIMEOUT_MS) {
      await new Promise((r) => setTimeout(r, HEALTH_POLL_INTERVAL_MS));
      const h = await probeHealth();
      if (h?.authenticated) {
        log('IB Key approved — /health.authenticated=true');
        return true;
      }
    }
    log('Timed out waiting for IB Key approval');
    return false;
  } catch (err) {
    log(`Login failed: ${(err as Error).message}`);
    return false;
  } finally {
    await ctx.close();
  }
}

// ---------- main ----------

function log(msg: string): void {
  console.log(`[${new Date().toISOString()}] [relogin] ${msg}`);
}

async function main(): Promise<void> {
  if (!USERNAME || !PASSWORD) {
    log('FATAL: IBKR_USERNAME or IBKR_PASSWORD not set in .env');
    process.exit(2);
  }

  if (await isDisabled()) {
    log('disabled sentinel present — exiting. Reset with: rm ~/.local/state/bezant-relogin/disabled');
    process.exit(0);
  }

  const health = await probeHealth();
  if (!health) {
    log(`Could not reach ${HEALTH_URL} — bezant-server may be down. Skipping login attempt.`);
    process.exit(0);
  }

  if (health.authenticated && health.connected) {
    log('IBKR session healthy — nothing to do');
    process.exit(0);
  }

  log(`Session unhealthy (authenticated=${health.authenticated} connected=${health.connected}) — starting login`);

  const state = await loadState();
  state.lastAttemptAt = new Date().toISOString();

  const browser = await chromium.launch({ headless: true });
  let success = false;
  try {
    success = await login(browser);
  } finally {
    await browser.close();
  }

  if (success) {
    state.consecutiveFailures = 0;
    state.lastSuccessAt = new Date().toISOString();
  } else {
    state.consecutiveFailures += 1;
  }

  if (!success && state.consecutiveFailures >= MAX_CONSECUTIVE_FAILURES) {
    await setDisabled();
    log(
      `Hit ${MAX_CONSECUTIVE_FAILURES} consecutive failures — disabling further automatic attempts. ` +
        `Manual reset required (see top-of-file comment).`,
    );
  }

  await saveState(state);
  process.exit(success ? 0 : 1);
}

main().catch((err) => {
  log(`Fatal: ${(err as Error).stack ?? err}`);
  process.exit(1);
});
