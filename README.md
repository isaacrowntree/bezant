# bezant

**Typed async access to the Interactive Brokers Client Portal Web API — Rust-first, with HTTP, CLI, MCP, and TypeScript surfaces auto-generated from the same vendored OpenAPI spec.**

[![CI](https://github.com/isaacrowntree/bezant/actions/workflows/ci.yml/badge.svg)](https://github.com/isaacrowntree/bezant/actions/workflows/ci.yml)
[![Docs](https://github.com/isaacrowntree/bezant/actions/workflows/docs.yml/badge.svg)](https://isaacrowntree.github.io/bezant/)
[![Audit](https://github.com/isaacrowntree/bezant/actions/workflows/audit.yml/badge.svg)](https://github.com/isaacrowntree/bezant/actions/workflows/audit.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![MSRV: 1.89](https://img.shields.io/badge/MSRV-1.89-orange.svg)](Cargo.toml)

> 📖 **Docs:** <https://isaacrowntree.github.io/bezant/> · 📦 **Issues:** <https://github.com/isaacrowntree/bezant/issues>

---

Bezant turns IBKR's 154-endpoint CPAPI into **five ergonomic surfaces** that all
ship from the same vendored OpenAPI 3.1 spec:

| Crate / package | Purpose |
|---|---|
| [`bezant`](crates/bezant-core) | Ergonomic async Rust facade — `Client`, keepalive, health, WebSocket streaming, pagination, symbol cache, typed errors |
| [`bezant-api`](crates/bezant-api) | Auto-generated Rust client covering **154 paths / 167 methods / 1030 types** |
| [`bezant-server`](crates/bezant-server) | HTTP sidecar — exposes CPAPI as plain REST+JSON so any language can consume it |
| [`bezant-cli`](crates/bezant-cli) | `bezant` CLI — `bezant health`, `bezant positions DU123456`, `bezant conid AAPL`, etc. |
| [`bezant-mcp`](crates/bezant-mcp) | Model Context Protocol server — exposes IBKR as MCP tools for Claude / Cursor / Continue |
| [`bezant-client`](clients/typescript) | TypeScript client — generated from the same spec for Node / Deno / browser |

All five are driven from one vendored spec. Re-run `./scripts/codegen.sh`
when IBKR ships a new revision and every surface updates together.

## Quickstart (Docker)

```sh
git clone https://github.com/isaacrowntree/bezant
cd bezant
docker compose up
```

- <https://localhost:5000> — log in to the IBKR Gateway once in a browser
- <http://localhost:8080/health> — verify the sidecar sees an authenticated session

```sh
curl http://localhost:8080/accounts
curl "http://localhost:8080/accounts/DU123456/positions?page=0"
```

Full quickstart for every surface: <https://isaacrowntree.github.io/bezant/quickstart.html>

## Rust

```toml
[dependencies]
bezant = { git = "https://github.com/isaacrowntree/bezant", package = "bezant-core" }
tokio = { version = "1", features = ["full"] }
```

```rust,no_run
use std::time::Duration;

#[tokio::main]
async fn main() -> bezant::Result<()> {
    let client = bezant::Client::new("https://localhost:5000/v1/api")?;
    let _keepalive = client.spawn_keepalive(Duration::from_secs(60));
    client.health().await?;

    // ergonomic helpers on top of the generated client
    let positions = client.all_positions("DU123456").await?;
    let aapl = bezant::SymbolCache::new(client).conid_for("AAPL").await?;
    println!("{} positions; AAPL = conid {aapl}", positions.len());
    Ok(())
}
```

## Highlights

- **Typed, async, reqwest-based client** covering every CPAPI endpoint.
- **Session keepalive** built in — no more 5-minute session expiries.
- **WebSocket streaming** with cookie auth reused from the REST session
  and typed subscribe helpers for market data / orders / PnL.
- **Enterprise-grade spec normalisation** (13 steps) works around real
  IBKR quirks — missing / duplicate `operationId`s, malformed
  `security[]`, integer fields with float example values, and more.
  Documented in detail at [Spec normalisation](https://isaacrowntree.github.io/bezant/internals/normalisation.html).
- **Snapshot tests** driven by real IBKR example payloads — catches
  upstream spec drift before our users do.
- **~100 tests** across the workspace (97 at last count), all green
  in CI: unit, integration, snapshot, and end-to-end against
  wiremock-mocked gateways.
- **Dual MIT / Apache-2.0** licensing following the Rust ecosystem convention.

## Production deployment: Cloudflare Zero Trust + Pi

Real-world IBKR API deploys hit a wall: **`api.ibkr.com` (fronted by
Akamai) rejects connections from cloud datacenter IPs**. So your
bezant-server can't run on Railway / Fly / Render / Heroku and reach the
upstream successfully — IBKR responds 401 to the SSO→CPAPI bridge call,
and every typed API call cascades into 401s.

The pattern that works in 2026:

```
Your bot (Railway/cloud)
   │   HTTPS + Service Token
   ▼
Cloudflare Zero Trust (Tunnel + Access)
   │   Cloudflare Tunnel
   ▼
Raspberry Pi at home
   │   ┌───────────────────────────────┐
   │   │  bezant-server (port 8080)   │
   │   │  CPGateway (port 5000)       │
   │   └───────────────────────────────┘
   │   Cloudflare WARP egress
   ▼
api.ibkr.com (Akamai)
```

**Why each piece:**
- **Pi at home** — residential ISP IP would also be flagged by Akamai
  *if it weren't for WARP*. (Don't skip WARP.)
- **Cloudflare WARP** on the Pi — routes outbound to api.ibkr.com
  through Cloudflare's edge IPs, which are reputationally clean.
- **Cloudflare Tunnel** — exposes the Pi to your bot without opening a
  port on your router or having a public IP.
- **Cloudflare Zero Trust Access** — gates the public hostname.
  Browsers (you) hit an SSO challenge; service-to-service calls
  (your bot) carry a Service Token in two headers.

### One-shot setup

```bash
# 1. On the Pi (Raspberry Pi 4/5 with 4GB+ RAM, Pi OS Lite arm64):
sudo apt update && curl -fsSL https://get.docker.com | sudo sh
sudo usermod -aG docker $USER

# 2. Install Cloudflare WARP (residential→clean-IP egress):
curl -fsSL https://pkg.cloudflareclient.com/pubkey.gpg | \
  sudo gpg --dearmor -o /usr/share/keyrings/cloudflare-warp-archive-keyring.gpg
echo "deb [signed-by=/usr/share/keyrings/cloudflare-warp-archive-keyring.gpg] \
  https://pkg.cloudflareclient.com/ bookworm main" | \
  sudo tee /etc/apt/sources.list.d/cloudflare-client.list
sudo apt update && sudo apt install -y cloudflare-warp
warp-cli --accept-tos registration new
warp-cli --accept-tos connect

# 3. Install Cloudflare Tunnel (cloudflared) — get a token from the
#    Zero Trust dashboard → Networks → Tunnels → Create:
sudo cloudflared service install <YOUR_TUNNEL_TOKEN>

# 4. Run bezant-combined (CPGateway + bezant-server in one container):
docker run -d --name bezant --restart unless-stopped \
  -p 127.0.0.1:8080:8080 \
  -e BEZANT_DEBUG_TOKEN="$(openssl rand -hex 32)" \
  ghcr.io/isaacrowntree/bezant-combined:latest
```

### Cloudflare dashboard configuration

1. **Tunnel** → add **Public Hostname**: `bezant.yourdomain.com` →
   service `HTTP localhost:8080`.
2. **Access** → **Applications** → add **Self-hosted** for the same
   hostname. Add two policies:
   - **Browser** (Allow): "Emails = you@yourdomain.com" — for you
     to do the IBKR login interactively.
   - **Service** (Service Auth): generated Service Token — for your
     bot. Save the **Client ID + Secret**.
3. Your bot calls `https://bezant.yourdomain.com/...` with two
   headers:
   ```
   CF-Access-Client-Id: <client-id>.access
   CF-Access-Client-Secret: <secret>
   ```

### Login flow

You'll need to do an interactive IBKR login periodically — open
`https://bezant.yourdomain.com/sso/Login` in a browser, get challenged
by Cloudflare Access SSO, then by IBKR's own login form, and approve
the 2FA push on your phone. Once that's done, bezant-server's
keepalive keeps the session warm and your bot's API calls succeed.

How often you need to re-login depends on IBKR — community reports
range from ~12h to a few days, and IBKR runs nightly maintenance that
typically forces a fresh login once per trading day. Don't assume a
hard SLA; design your bot to handle a 401 by surfacing a "needs
login" alert rather than crashing.

### Security model

- **Cloudflare Zero Trust is the primary perimeter.** With a correctly
  configured Access policy, only your email-authenticated browser and
  your token-authenticated bot can reach the Pi.
- **bezant-server's `BEZANT_BIND` defaults to `0.0.0.0:8080`** — that's
  fine *behind* Cloudflare Tunnel + a `127.0.0.1` Docker port-bind
  (as in the snippet above). Don't expose 8080 directly to the
  internet without Zero Trust in front.
- **Debug endpoints (`/debug/jar`, `/debug/probe`)** are off by
  default. Setting `BEZANT_DEBUG_TOKEN` enables them, gated by an
  `X-Bezant-Debug-Token` header (or `?token=…` query). With Zero
  Trust in front, this is defense-in-depth — leave it off until
  you've verified your Access policies are tight.
- **The shared cookie jar holds live IBKR session cookies.** Anyone
  who can read it can resume the IBKR session and trade your account.
  bezant-server is **single-tenant by design** — don't deploy this
  proxy multi-tenant.

## Repository layout

```
crates/
  bezant-spec/     — vendored IBKR OpenAPI spec + refresh tooling
  bezant-api/      — auto-generated Rust client (don't hand-edit)
  bezant-core/     — ergonomic facade
  bezant-server/   — axum HTTP sidecar
  bezant-cli/      — clap-powered CLI
  bezant-mcp/      — MCP server
clients/
  typescript/      — auto-generated TS client
scripts/           — spec pipeline (normalise → upgrade → codegen)
docs/              — mdbook source for the docs site
xtask/             — dev-only tools (spec probing / bisection)
.github/           — CI, release, docs, audit workflows
```

## Contributing

We welcome contributions — especially spec-normaliser improvements and
new client languages. See [CONTRIBUTING.md](CONTRIBUTING.md) for the dev
workflow, or open a discussion on GitHub if you want to talk architecture.

## Status

**Alpha — v0.1.** Works end-to-end against IBKR paper accounts. API
surface will evolve until v1.0. See the
[ROADMAP](ROADMAP.md) for what's next.

## Security

See [SECURITY.md](SECURITY.md) for disclosure policy. **Do not open a
public issue for vulnerabilities** — email the maintainer instead.

## License

Dual-licensed under your choice of:

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

Contributions are assumed to be offered under the same terms.

The **vendored IBKR OpenAPI spec** itself is Interactive Brokers' IP;
included under fair-use conventions for interoperability.

## Not affiliated with Interactive Brokers

Bezant is an independent open-source project. Trading involves substantial
risk; this software is provided **without warranty**. See the license text.
