#!/usr/bin/env bash
# Run the IBKR Client Portal Gateway and bezant-server together. Either
# process exiting terminates the whole container — Railway (or whoever)
# restarts us. tini (PID 1) reaps the shell, the shell reaps the children.

set -e

echo "[entrypoint] starting combined sidecar (gateway + bezant-server)" >&2
cd /gw

# When the Gateway is fronted by a reverse proxy that lives on a
# different public hostname (Railway, fly.io, etc.), the empty default
# `portalBaseURL` makes CPGateway's CPAPI handlers reject post-login
# requests with 401 because the browser-supplied Origin/Referer don't
# match what the Gateway thinks it served. Rewrite the conf at boot
# from the `PORTAL_BASE_URL` env var (or, if unset, from
# `RAILWAY_PUBLIC_DOMAIN` which Railway injects automatically).
PORTAL_URL="${PORTAL_BASE_URL:-}"
if [ -z "$PORTAL_URL" ] && [ -n "${RAILWAY_PUBLIC_DOMAIN:-}" ]; then
    PORTAL_URL="https://${RAILWAY_PUBLIC_DOMAIN}"
fi
if [ -n "$PORTAL_URL" ]; then
    echo "[entrypoint] pinning portalBaseURL to $PORTAL_URL" >&2
    # YAML's quoting is forgiving — wrap in double quotes and escape `/`
    # and `&` for sed safety.
    ESCAPED=$(printf '%s' "$PORTAL_URL" | sed -e 's/[\/&]/\\&/g')
    sed -i "s|portalBaseURL: \"\"|portalBaseURL: \"$ESCAPED\"|" root/conf.yaml
fi

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

cleanup() {
    kill -TERM "$GW_PID" 2>/dev/null || true
    kill -TERM "$BZ_PID" 2>/dev/null || true
    wait
}
trap cleanup TERM INT

wait -n "$GW_PID" "$BZ_PID"
EXIT_CODE=$?
cleanup
exit "$EXIT_CODE"
