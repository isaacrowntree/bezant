# bezant-watchdog

Pi-side liveness watchdog for the `bezant` Docker container.

## What it does

Runs once a minute. Probes `http://localhost:8080/health` and inspects
`bezant-relogin`'s state files. If the container is in a state that's
empirically required a manual `docker restart bezant` to recover, the
watchdog does it automatically and clears the relogin disabled flag so
the next 5-min relogin tick fires fresh.

## Restart triggers

| Trigger | Threshold | Why |
|---|---|---|
| `/health` returns 5xx or unreachable | 5 consecutive probes (~5 min) | Server crashed but container PID still alive, or networking stack wedged |
| `bezant-relogin/disabled` sentinel exists | immediate | Relogin tried 3 times and gave up — either user genuinely couldn't tap, or bezant/CPGateway is in the stuck state where logins succeed in the browser but typed `/health` keeps reporting authenticated=false |

## What does NOT trigger a restart

- `/health` returns 401 not_authenticated alone — that's the normal
  state immediately after session expiry; the relogin timer handles it
- A single 5xx — could be transient
- High relogin failure count without the disabled sentinel — relogin
  will get there on its own after MAX_CONSECUTIVE_FAILURES

## Cooldown

**2 hours** minimum between restarts. If the user genuinely isn't
around to tap an IB Key push, restarting bezant in a tight loop won't
help — relogin will just disable itself again and we'd loop. Two hours
gives enough buffer for the user to surface and tap.

## Pi setup

```bash
ssh zackdesign-pi
cd ~/bezant && git pull
ln -sf ~/bezant/watchdog ~/bezant-watchdog
cd ~/bezant-watchdog && npm install

mkdir -p ~/.config/systemd/user
cp systemd/bezant-watchdog.service ~/.config/systemd/user/
cp systemd/bezant-watchdog.timer ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable --now bezant-watchdog.timer
```

(linger should already be enabled from bezant-relogin setup; if not:
`sudo loginctl enable-linger isaac`)

## Tail logs

```bash
journalctl --user -u bezant-watchdog -f
```

You'll see one line per minute like:

```
[2026-05-05T00:55:01.234Z] [watchdog] status: health=authenticated relogin_failures=0 (healthy)
```

When something fires:

```
[2026-05-05T05:30:01.000Z] [watchdog] /health transition: authenticated → not_authenticated
[2026-05-05T05:35:01.000Z] [watchdog] RESTARTING bezant: bezant-relogin self-disabled (3 consecutive failures) — bouncing bezant to clear potentially-wedged session state
[2026-05-05T05:35:08.000Z] [watchdog] docker restart returned successfully — waiting for /health to respond
[2026-05-05T05:35:13.000Z] [watchdog] Post-restart /health responsive: not_authenticated
[2026-05-05T05:35:13.000Z] [watchdog] Cleared bezant-relogin disabled sentinel — next 5-min relogin tick will retry
```

## State

Persisted at `~/.local/state/bezant-watchdog/state.json`:

```json
{
  "lastHealthState": "authenticated",
  "consecutiveServerErrors": 0,
  "lastRestartAt": null,
  "lastRestartReason": null,
  "totalRestarts": 0
}
```

## Tuning

All thresholds are env-overridable. Defaults (top of `index.ts`):

- `BEZANT_HEALTH_URL` — default `http://localhost:8080/health`
- `BEZANT_CONTAINER` — default `bezant`
- `BEZANT_RELOGIN_DISABLED_FILE` — default `~/.local/state/bezant-relogin/disabled`
- `BEZANT_RELOGIN_STATE_FILE` — default `~/.local/state/bezant-relogin/state.json`
- `BEZANT_WATCHDOG_STATE_DIR` — default `~/.local/state/bezant-watchdog`

The thresholds (server-error count, restart cooldown) live as `const`s
in `index.ts` — change there if you need different behavior.

## Manual override

Force an immediate restart attempt (ignores cooldown — you'll need to
delete the state file or wait):

```bash
ssh zackdesign-pi 'docker restart bezant'
```

Force the watchdog to run a probe right now (instead of waiting for
the next 60s tick):

```bash
ssh zackdesign-pi 'systemctl --user start bezant-watchdog.service'
```
