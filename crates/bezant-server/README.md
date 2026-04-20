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

## Endpoints (v0.1)

| Method | Path                                   | Upstream CPAPI                                 |
|--------|----------------------------------------|------------------------------------------------|
| GET    | `/health`                              | `POST /iserver/auth/status`                    |
| GET    | `/accounts`                            | `GET /portfolio/accounts`                      |
| GET    | `/accounts/:id/summary`                | `GET /portfolio/{id}/summary`                  |
| GET    | `/accounts/:id/positions?page=N`       | `GET /portfolio/{id}/positions/{N}`            |
| GET    | `/accounts/:id/ledger`                 | `GET /portfolio/{id}/ledger`                   |
| GET    | `/contracts/search?symbol=X`           | `POST /iserver/secdef/search`                  |
| GET    | `/market/snapshot?conids=A,B&fields=…` | `GET /iserver/marketdata/snapshot?…`           |

All upstream responses are returned verbatim (JSON body + original status).
Auth errors map to `401 not_authenticated` / `503 no_session`.

## Running

```sh
cargo run -p bezant-server -- \
  --gateway-url https://localhost:5000/v1/api \
  --bind 0.0.0.0:8080
```

Or via env vars: `IBKR_GATEWAY_URL`, `BEZANT_BIND`, `BEZANT_KEEPALIVE_SECS`.

## License

Dual-licensed under MIT or Apache-2.0 at your option.
