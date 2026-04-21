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
# wiring routes, which makes bezant-server's first /tickle race.
for _ in $(seq 1 30); do
    if curl -sk -o /dev/null -w '%{http_code}' https://127.0.0.1:5000/v1/api/iserver/auth/status | grep -qE '^[234]'; then
        break
    fi
    sleep 1
done

echo "[entrypoint] Gateway is responsive, starting bezant-server" >&2
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
