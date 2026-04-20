# bezant-cli

Command-line interface to the IBKR Client Portal Web API.

Wraps the Rust [`bezant`](../bezant-core) crate in a clap-powered binary so
ops users, shell scripts, and ad-hoc sessions can poke the Gateway without
writing Rust or spinning up the HTTP sidecar.

Output is JSON on stdout; pass `--pretty` for indented JSON. Errors land on
stderr with a non-zero exit code.

## Install

Until the crate is on crates.io:

```sh
cargo install --git https://github.com/isaacrowntree/bezant bezant-cli
```

Then make sure the Client Portal Gateway is running locally (via
`docker compose up` from the repo root, or the official IBKR installer).

## Common uses

```sh
# Check session state
bezant health

# List available accounts
bezant accounts --pretty

# Account summary
bezant summary DU123456 --pretty

# All positions (pagination handled for you)
bezant positions DU123456 --pretty

# Ticker → conid lookup
bezant conid AAPL

# Keep the session alive (useful in a cron)
bezant tickle
```

## Environment

| Variable                    | Default                             | Meaning                                    |
|-----------------------------|-------------------------------------|--------------------------------------------|
| `IBKR_GATEWAY_URL`          | `https://localhost:5000/v1/api`     | Gateway base URL                           |
| `BEZANT_REJECT_INVALID_CERTS` | unset (accepts self-signed)       | Set to `1` to enforce TLS cert validation  |
| `RUST_LOG`                  | `warn`                              | Standard tracing env-filter                |

## License

Dual-licensed under MIT or Apache-2.0 at your option.
