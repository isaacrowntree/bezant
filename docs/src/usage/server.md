# HTTP sidecar (`bezant-server`)

A thin axum binary that exposes the CPAPI as plain REST+JSON. Most of its
handlers are deliberately *pass-through* — they forward the Gateway's JSON
body verbatim — so any language can consume CPAPI without linking Rust.

## Endpoints

### REST passthrough

| Method     | Path                                     | Upstream                                       |
|------------|------------------------------------------|------------------------------------------------|
| GET        | `/health`                                | `POST /iserver/auth/status` (projected)        |
| GET        | `/accounts`                              | `GET /portfolio/accounts`                      |
| GET        | `/accounts/:id/summary`                  | `GET /portfolio/{id}/summary`                  |
| GET        | `/accounts/:id/positions?page=N`         | `GET /portfolio/{id}/positions/{N}`            |
| GET        | `/accounts/:id/ledger`                   | `GET /portfolio/{id}/ledger`                   |
| GET / POST | `/accounts/:id/orders`                   | `GET / POST /iserver/account/{id}/orders`      |
| DELETE     | `/accounts/:id/orders/:order_id`         | `DELETE /iserver/account/{id}/order/{oid}`     |
| GET        | `/contracts/search?symbol=X`             | `POST /iserver/secdef/search`                  |
| GET        | `/market/snapshot?conids=A,B&fields=…`   | `GET /iserver/marketdata/snapshot?…`           |
| *fallback* | any other path                           | verbatim passthrough (drives `/sso/Login` etc.) |

### Events capture (opt-in via `BEZANT_EVENTS_ENABLED`)

The server can optionally run an internal CPAPI WebSocket consumer that
buffers order, PnL, and (lazily per-conid) market-data frames into
per-topic ring buffers. Consumers poll cursor-paginated REST endpoints
instead of opening their own WebSocket — events are captured server-side
the moment they arrive, regardless of whether anyone is currently
listening.

| Method | Path                                          | Returns                                              |
|--------|-----------------------------------------------|------------------------------------------------------|
| GET    | `/events/orders?since=N&limit=N`              | order lifecycle frames (CPAPI `sor`)                 |
| GET    | `/events/pnl?since=N&limit=N`                 | PnL frames (CPAPI `spl`)                             |
| GET    | `/events/marketdata?conid=N&since=N&limit=N`  | L1 market data; lazy upstream subscribe per conid    |
| GET    | `/events/gap?since=N&limit=N`                 | synthetic gap markers (WS reconnect, process restart) |
| GET    | `/events/_status`                             | connector liveness + per-topic buffer sizes          |
| GET    | `/events/{topic}/history?since_ts=…&limit=N`  | sqlite history (when `BEZANT_EVENTS_DB_PATH` is set) |

Wire semantics:

- **200** — `{events, next_cursor, reset_epoch}`. Use `next_cursor` as
  the next `since=`.
- **204** — caller is caught up; cursor stays put.
- **412** — `{head_cursor, reset_epoch, code: "cursor_expired"}`. The
  caller's cursor is older than the ring buffer's head; reset to
  `head_cursor - 1` and emit a synthetic gap on the consumer side.
- **503** — `{code: "events_disabled"}` when capture is off.

`reset_epoch` bumps on every WS reconnect or process restart. Any change
in epoch is the consumer's signal that "you missed something" — the
connector also injects a synthetic event into every active topic ring
so a polling consumer sees the gap on its next read.

## Error envelope

Non-success responses come back as:

```json
{ "code": "not_authenticated", "message": "gateway is not authenticated …" }
```

Status codes map:

| Variant | HTTP |
|---|---|
| `not_authenticated` | 401 |
| `no_session` | 503 |
| `upstream_http_error` | 502 |
| `upstream_api_error` | 502 |
| `invalid_base_url` | 400 |
| `internal` | 500 |

## Configuration

Env-first, clap-exposed. See `bezant-server --help`.

| Variable                            | Default                              |
|-------------------------------------|--------------------------------------|
| `IBKR_GATEWAY_URL`                  | `https://localhost:5000/v1/api`      |
| `BEZANT_BIND`                       | `0.0.0.0:8080`                       |
| `BEZANT_KEEPALIVE_SECS`             | `60`                                 |
| `BEZANT_VERIFY_TLS`                 | `false` (accepts the Gateway's self-signed cert) |
| `BEZANT_DEBUG_TOKEN`                | unset (`/debug/*` 404s without it)   |
| `BEZANT_EVENTS_ENABLED`             | `false`                              |
| `BEZANT_EVENTS_DB_PATH`             | unset (sqlite history disabled)      |
| `BEZANT_EVENTS_ORDERS_CAP`          | `1000`                               |
| `BEZANT_EVENTS_PNL_CAP`             | `5000`                               |
| `BEZANT_EVENTS_MARKETDATA_CAP`      | `2000` per conid                     |

## Deployment shape

The Docker compose file in the repo root is the canonical shape:

```
┌────────────┐  stdin/stdout   ┌──────────────┐  HTTPS + cookie   ┌──────┐
│ your app   │ ──────────────► │ bezant-server│ ────────────────► │ IBKR │
│ (any lang) │ ◄────────────── │              │ ◄──────────────── │  GW  │
└────────────┘    HTTP/JSON    └──────────────┘                   └──────┘
```

Tip: keep the sidecar on `127.0.0.1` in production. It holds a live IBKR
session cookie — anyone who reaches its port can make trades.
