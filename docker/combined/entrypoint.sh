#!/usr/bin/env bash
# Run the IBKR Client Portal Gateway and bezant-server together. Either
# process exiting terminates the whole container — Railway (or whoever)
# restarts us. tini (PID 1) reaps the shell, the shell reaps the children.

set -e

echo "[entrypoint] starting combined sidecar (gateway + bezant-server)" >&2
cd /gw

# Tee the Gateway's stdout+stderr to our own so Railway captures it even
# if Java decides to re-open file descriptors.
bin/run.sh root/conf.yaml 2>&1 &
GW_PID=$!
echo "[entrypoint] Gateway pid=$GW_PID, probing :5000" >&2

# Wait up to 60s for the Gateway to bind :5000 before launching bezant-server.
# A naked /dev/tcp probe is enough — we only need the port open, not a valid
# CPAPI response.
for _ in $(seq 1 60); do
    if (exec 3<>/dev/tcp/127.0.0.1/5000) 2>/dev/null; then
        exec 3<&- 3>&-
        break
    fi
    sleep 1
done

if ! kill -0 "$GW_PID" 2>/dev/null; then
    echo "[entrypoint] Gateway process died before :5000 came up" >&2
    exit 1
fi

# One final HTTPS probe: the TCP port can be open while Jetty is still
# wiring routes, which makes bezant-server's first /tickle race. Bound
# each individual curl with `--max-time 3` so a Gateway that's slow to
# respond (or a `portalBaseURL` redirect that turns the probe into an
# infinite loop) doesn't wedge the entrypoint past the loop's overall
# 30s budget.
for _ in $(seq 1 30); do
    if curl -sk --max-time 3 -o /dev/null -w '%{http_code}' \
        https://127.0.0.1:5000/v1/api/iserver/auth/status \
        | grep -qE '^[234]'; then
        break
    fi
    sleep 1
done

echo "[entrypoint] Gateway is responsive (or probe gave up), starting bezant-server" >&2
/usr/local/bin/bezant-server &
BZ_PID=$!

# ── SSO bridge watchdog ──────────────────────────────────────────────────────
# The Gateway can be ALIVE AND WRONG, and this container had no way to notice.
#
# Observed 2026-09-03: `iserver/auth/ssodh/init` returned HTTP 500 for eleven
# hours straight. That endpoint is the SSO handshake — the step that turns a
# completed browser login into an authenticated session — so no login could
# possibly succeed. Three correct 2FA response codes and several IB Key taps
# were spent before anyone thought to look at it. `/health`, `auth/status` and
# the process table all looked fine the entire time, and the supervision below
# (`wait -n`) only fires when a process EXITS. A restart cleared it instantly,
# after which the same endpoint answered 401 like it should.
#
# IBKR has not shipped a new Gateway build since 2023-04-24 — stable and beta
# are the same afternoon — so this is a permanent operating condition, not a
# bug awaiting an upstream patch. Recovering from it automatically is the fix.
#
# ONLY while logged out. A restart destroys an authenticated session and costs
# an IB Key tap, which is the most expensive thing in this system; the observed
# wedge happened while logged out, where a bounce is free. If it ever wedges
# while authenticated we want a human, not a reboot loop.
SSO_WEDGE_CHECK_SECS="${SSO_WEDGE_CHECK_SECS:-60}"
SSO_WEDGE_THRESHOLD="${SSO_WEDGE_THRESHOLD:-5}"

sso_wedge_watch() {
    local faults=0 auth_code sso_code
    # Give the Gateway a generous grace period before the first verdict: it is
    # slow to become fully functional after a start, and a watchdog that bounces
    # a container that was merely still waking up is worse than none.
    sleep 120
    while :; do
        sleep "$SSO_WEDGE_CHECK_SECS"
        # Are we logged out? Only then is a restart free.
        auth_code=$(curl -sk --max-time 5 -o /dev/null -w '%{http_code}' \
            https://127.0.0.1:5000/v1/api/iserver/auth/status 2>/dev/null || echo 000)
        # 401/403 => logged out. 2xx => authenticated, leave it alone entirely.
        case "$auth_code" in
            2*) faults=0; continue ;;
            000) continue ;;   # unreachable is the other supervisor's problem
        esac

        sso_code=$(curl -sk --max-time 10 -o /dev/null -w '%{http_code}' \
            -X POST -H 'Content-Type: application/json' \
            -d '{"publish":true,"compete":true}' \
            https://127.0.0.1:5000/v1/api/iserver/auth/ssodh/init 2>/dev/null || echo 000)

        case "$sso_code" in
            5*) faults=$((faults + 1))
                echo "[entrypoint] SSO bridge fault ${faults}/${SSO_WEDGE_THRESHOLD} (ssodh/init HTTP ${sso_code}, auth ${auth_code})" >&2 ;;
            000) ;;                      # transport blip, do not count or reset
            *)  if [ "$faults" -gt 0 ]; then
                    echo "[entrypoint] SSO bridge recovered (HTTP ${sso_code})" >&2
                fi
                faults=0 ;;
        esac

        if [ "$faults" -ge "$SSO_WEDGE_THRESHOLD" ]; then
            echo "[entrypoint] SSO BRIDGE WEDGED — ssodh/init returned 5xx ${faults}x while logged out." >&2
            echo "[entrypoint] No login can complete in this state. Killing the Gateway so the container restarts." >&2
            # Kill the Gateway rather than exiting this shell directly: `wait -n`
            # below is already watching it, so this routes through the existing
            # shutdown path instead of adding a second way for the container to die.
            kill -TERM "$GW_PID" 2>/dev/null || true
            return
        fi
    done
}

sso_wedge_watch &
SSO_WATCH_PID=$!

cleanup() {
    kill -TERM "$SSO_WATCH_PID" 2>/dev/null || true
    kill -TERM "$GW_PID" 2>/dev/null || true
    kill -TERM "$BZ_PID" 2>/dev/null || true
    wait
}
trap cleanup TERM INT

wait -n "$GW_PID" "$BZ_PID"
EXIT_CODE=$?
cleanup
exit "$EXIT_CODE"
