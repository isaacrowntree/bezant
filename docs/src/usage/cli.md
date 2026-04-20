# Command-line (`bezant-cli`)

Ships as the `bezant` binary. Every subcommand prints JSON on stdout;
pass `--pretty` for indented output. Errors exit non-zero with a `bezant:`
prefix on stderr.

## Install

```sh
cargo install --git https://github.com/isaacrowntree/bezant bezant-cli
```

## Subcommands

```sh
bezant health                         # auth + session status
bezant tickle                         # extend the session manually
bezant accounts --pretty              # list accounts
bezant summary DU123456 --pretty      # portfolio summary
bezant positions DU123456 --pretty    # paginated positions (all pages)
bezant conid AAPL                     # ticker → conid lookup
```

## Scripting

Every subcommand produces stable JSON, so `jq` is your friend:

```sh
bezant accounts | jq -r '.[].accountId'
bezant positions DU123456 | jq 'map(select(.position > 0))'
```

## Environment

| Variable                        | Default                           |
|---------------------------------|-----------------------------------|
| `IBKR_GATEWAY_URL`              | `https://localhost:5000/v1/api`   |
| `BEZANT_REJECT_INVALID_CERTS`   | unset (accepts self-signed)       |
| `RUST_LOG`                      | `warn`                            |
