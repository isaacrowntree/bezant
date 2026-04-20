# Railway / cloud deployment

The Docker compose stack translates cleanly to any container platform.
Notes for Railway (what we tested against) and general guidance.

## Railway

Split into two services:

1. **`ib-gateway`** — use the `ghcr.io/gnzsnz/clientportal` image, pin by
   digest. Private networking only; don't expose port 5000 publicly.
2. **`bezant-server`** — build from this repo's `Dockerfile`. Set
   `IBKR_GATEWAY_URL=https://ib-gateway.railway.internal:5000/v1/api` so
   it reaches the Gateway over Railway's private network.

You will still need to log in to the Gateway once via a VNC / RDP tunnel
to complete the initial IBKR 2FA. The Gateway keeps the session alive
after that; Bezant keeps it tickled.

## Why pass-through

`bezant-server`'s handlers forward the Gateway's JSON body verbatim —
they don't decode into typed Rust structs and re-encode as JSON. Three
reasons:

1. **No schema drift on the hot path.** If IBKR adds a new field to
   `portfolio/summary`, your consumers see it immediately with zero code
   changes in bezant.
2. **Smaller attack surface.** Pass-through means the sidecar can't
   accidentally strip fields or round-trip floats incorrectly.
3. **Faster.** No double decode; just stream bytes.

The *typed* layer is `bezant-api`, which we consciously keep separate.
Rust consumers that want typed access link the crate directly; anyone
going over HTTP just needs JSON.

## Secrets

- **Gateway login** — ideally stays in the Gateway image's config, bound
  to the account owner's 2FA device. Don't paste IBKR passwords into
  container env vars.
- **`RELEASE_PLZ_TOKEN` / `CARGO_REGISTRY_TOKEN`** — repo secrets for the
  GitHub Actions release workflow (optional until we publish to crates.io).

## Health checks

The Gateway exposes `/v1/api/iserver/auth/status`. `bezant-server` exposes
`/health`. Hook your platform's health check to `/health` — it returns
`{"authenticated": true, ...}` with HTTP 200 only when IBKR is happy.
