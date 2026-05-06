# bezant-server

HTTP sidecar exposing the IBKR Client Portal Web API over REST.

Runs next to the IBKR Client Portal Gateway (in the same Docker network,
pod, or host) and gives any language — Node, Python, Go, shell — a clean
REST/JSON surface to talk to IBKR, without needing to link Rust.

```
┌──────────────┐  HTTP/JSON  ┌────────────────┐  HTTPS+cookie  ┌─────────┐
│  your bot    │───────────► │ bezant-server  │───────────────►│ IBKR    │
│ (any lang)   │             │  (this binary) │                │ Gateway │
└──────────────┘             └────────────────┘                └─────────┘
```

## Endpoints

### REST passthrough

| Method        | Path                                       | Upstream CPAPI                                  |
|---------------|--------------------------------------------|-------------------------------------------------|
| GET           | `/health`                                  | `POST /iserver/auth/status` (projected)         |
| GET           | `/accounts`                                | `GET /portfolio/accounts`                       |
| GET           | `/accounts/:id/summary`                    | `GET /portfolio/{id}/summary`                   |
| GET           | `/accounts/:id/positions?page=N`           | `GET /portfolio/{id}/positions/{N}`             |
| GET           | `/accounts/:id/ledger`                     | `GET /portfolio/{id}/ledger`                    |
| GET / POST    | `/accounts/:id/orders`                     | `GET / POST /iserver/account/{id}/orders`       |
| DELETE        | `/accounts/:id/orders/:order_id`           | `DELETE /iserver/account/{id}/order/{oid}`      |
| GET           | `/contracts/search?symbol=X`               | `POST /iserver/secdef/search`                   |
| GET           | `/market/snapshot?conids=A,B&fields=…`     | `GET /iserver/marketdata/snapshot?…`            |
| GET           | `/debug/jar` (token-gated)                 | cookie jar dump (names + lengths only)          |
| GET           | `/debug/probe` (token-gated)               | walks `auth/status → accounts` for diagnosis    |
| *fallback*    | any other path                             | verbatim passthrough (drives `/sso/Login` etc.) |

All upstream responses are returned verbatim (JSON body + original status).
Auth errors map to `401 not_authenticated` / `503 no_session`.

### Events capture (opt-in)

When started with `BEZANT_EVENTS_ENABLED=true`, the server runs an internal
WebSocket consumer against `/v1/api/ws` that captures order, PnL, and (lazily
per-conid) market-data frames into per-topic ring buffers. Cursor-paginated
REST reads:

| Method | Path                                          | Returns                                            |
|--------|-----------------------------------------------|----------------------------------------------------|
| GET    | `/events/orders?since=N&limit=N`              | order lifecycle frames (sor)                       |
| GET    | `/events/pnl?since=N&limit=N`                 | PnL update frames (spl)                            |
| GET    | `/events/marketdata?conid=N&since=N&limit=N`  | L1 market data; lazily subscribes on first request |
| GET    | `/events/gap?since=N&limit=N`                 | synthetic gap markers (reconnect / process restart) |
| GET    | `/events/_status`                             | connector liveness + per-topic buffer sizes        |
| GET    | `/events/{topic}/history?since_ts=…&limit=N`  | sqlite-backed history (when `BEZANT_EVENTS_DB_PATH` set) |

Wire semantics: 200 with `{events, next_cursor, reset_epoch}` on hit, 204
on caught-up, 412 with `{head_cursor, reset_epoch}` when the caller's
cursor has fallen past the ring buffer's head (consumer should reset
to head and emit a synthetic gap on its side).

When events capture is off, `/events/*` returns 503 `events_disabled`.

## Running

```sh
cargo run -p bezant-server -- \
  --gateway-url https://localhost:5000/v1/api \
  --bind 0.0.0.0:8080 \
  --events-enabled \
  --events-db-path /var/lib/bezant-events/events.db
```

## Configuration

| Variable                            | Default                              | Notes                                        |
|-------------------------------------|--------------------------------------|----------------------------------------------|
| `IBKR_GATEWAY_URL`                  | `https://localhost:5000/v1/api`      |                                              |
| `BEZANT_BIND`                       | `0.0.0.0:8080`                       |                                              |
| `BEZANT_KEEPALIVE_SECS`             | `60`                                 | tickle cadence                               |
| `BEZANT_VERIFY_TLS`                 | `false`                              | accept Gateway's self-signed cert by default |
| `BEZANT_DEBUG_TOKEN`                | unset                                | enables `/debug/*` when set                  |
| `BEZANT_EVENTS_ENABLED`             | `false`                              | enables the WS event consumer + `/events/*`  |
| `BEZANT_EVENTS_DB_PATH`             | unset                                | sqlite path for `/events/{topic}/history`    |
| `BEZANT_EVENTS_ORDERS_CAP`          | `1000`                               | orders ring capacity                         |
| `BEZANT_EVENTS_PNL_CAP`             | `5000`                               | pnl ring capacity                            |
| `BEZANT_EVENTS_MARKETDATA_CAP`      | `2000`                               | per-conid marketdata ring capacity           |

## License

Dual-licensed under MIT or Apache-2.0 at your option.
