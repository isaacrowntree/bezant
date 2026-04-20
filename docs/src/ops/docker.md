# Docker deployment

The repo ships a `docker-compose.yml` that brings up the IBKR Client Portal
Gateway and `bezant-server` together. This is the canonical local setup.

```sh
docker compose up
```

Then:

- <https://localhost:5000> — log in to the Gateway once in a browser
- <http://localhost:8080/health> — sanity-check the sidecar

## What the image contains

- `Dockerfile` builds the Rust workspace with `rust:1.82-bookworm`.
- Final runtime image is `gcr.io/distroless/cc-debian12:nonroot` — about
  20 MB, no shell, no package manager, minimal attack surface.
- Only the `bezant-server` binary is copied in. No IBKR Gateway inside the
  image — that runs in the sibling compose service.

## Binding

Both services bind to `127.0.0.1` in the compose file:

```yaml
ports:
  - "127.0.0.1:5000:5000"   # Gateway
  - "127.0.0.1:8080:8080"   # bezant-server
```

**Keep it that way in production.** The Gateway holds a live IBKR session
cookie; the sidecar has no auth in front of it. Reaching either port means
trading power. If you need remote access, tunnel through SSH or put a
proper auth proxy (oauth2-proxy, caddy + basic auth, cloudflared tunnel)
in front.

## Image pinning

Pin the Gateway image by digest for reproducible deployments:

```yaml
image: ghcr.io/gnzsnz/clientportal@sha256:<digest>
```

Update the pin when IBKR ships a new Gateway release.
