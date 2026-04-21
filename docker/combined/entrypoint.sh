#!/usr/bin/env bash
# Run the IBKR Client Portal Gateway and bezant-server together. Either
# process exiting terminates the whole container — Railway (or whoever)
# restarts us. tini (PID 1) reaps the shell, the shell reaps the children.

set -e

cd /gw
bin/run.sh root/conf.yaml &
GW_PID=$!

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
    echo "[entrypoint] Gateway failed to start within 60s" >&2
    exit 1
fi

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
