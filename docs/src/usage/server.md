# HTTP sidecar (`bezant-server`)

A thin axum binary that exposes the CPAPI as plain REST+JSON. Most of its
handlers are deliberately *pass-through* — they forward the Gateway's JSON
body verbatim — so any language can consume CPAPI without linking Rust.

## Endpoints

| Method | Path | Upstream |
|---|---|---|
| GET | `/health` | `POST /iserver/auth/status` (projected) |
| GET | `/accounts` | `GET /portfolio/accounts` |
| GET | `/accounts/:id/summary` | `GET /portfolio/{id}/summary` |
| GET | `/accounts/:id/positions?page=N` | `GET /portfolio/{id}/positions/{N}` |
| GET | `/accounts/:id/ledger` | `GET /portfolio/{id}/ledger` |
| GET | `/contracts/search?symbol=X` | `POST /iserver/secdef/search` |
| GET | `/market/snapshot?conids=A,B&fields=…` | `GET /iserver/marketdata/snapshot?…` |

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

| Variable                        | Default                           |
|---------------------------------|-----------------------------------|
| `IBKR_GATEWAY_URL`              | `https://localhost:5000/v1/api`   |
| `BEZANT_BIND`                   | `0.0.0.0:8080`                    |
| `BEZANT_KEEPALIVE_SECS`         | `60`                              |
| `BEZANT_REJECT_INVALID_CERTS`   | unset (accepts self-signed)       |

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
