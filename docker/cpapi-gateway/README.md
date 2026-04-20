# cpapi-gateway

Minimal container image wrapping IBKR's official Client Portal Gateway
(the Java app you'd otherwise download and run locally).

Published to `ghcr.io/isaacrowntree/cpapi-gateway`. Pairs with
[`bezant-server`](https://ghcr.io/isaacrowntree/bezant-server) —
`bezant-server` talks CPAPI over this gateway's :5000 port, and the
user logs in once via browser.

## Why not IBeam / IBC

IBeam's Selenium-based auto-login breaks whenever IBKR tweaks the login
page. For deployments that tolerate a one-time manual login per restart
(which is most of them — restarts are rare), a plain gateway is simpler
and more reliable.

For true zero-touch restarts, use IBeam instead. Bezant works the same
either way.

## Usage

```sh
docker run -p 127.0.0.1:5000:5000 ghcr.io/isaacrowntree/cpapi-gateway:latest
```

Open <https://localhost:5000> → log in with IBKR credentials + 2FA.
Keep the container running.

## Rebuilding the image

`scripts/docker.yml` in this repo's root publishes this image on every
push to `main`. To build locally:

```sh
docker build -t cpapi-gateway docker/cpapi-gateway
```

## License note

The Docker image baked from IBKR's distributable remains IBKR's
intellectual property. This Dockerfile is a deployment convenience
published under the repo's MIT/Apache-2.0 license, but the embedded
Gateway artefact is not.
