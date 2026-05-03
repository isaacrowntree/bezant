# Combined cpapi-gateway + bezant-server image.
#
# For single-service deploys (a residential Pi behind Cloudflare Tunnel,
# Railway, fly.io, a bare VM) where both processes always live together:
# the IBKR Client Portal Gateway on 127.0.0.1:5000 and bezant-server
# fronting it on :8080. The user hits bezant-server for both the
# interactive login UI (via fallback passthrough) and the JSON API —
# the cookie lands in bezant-server's reqwest jar, which is what
# authenticated API calls use.
#
# Consumers who want to run the two separately can still use the
# standalone images in ../cpapi-gateway/ and the repo-root Dockerfile.
#
# Build speed: this Dockerfile is structured for layer-friendly
# incremental builds — `cargo chef` materialises a dependency-only
# layer that's cached across source-only changes, and BuildKit cache
# mounts (`--mount=type=cache`) persist `/usr/local/cargo/registry`
# and `/work/target` across `docker build` invocations.
#
# syntax=docker/dockerfile:1.7

# ---- chef base ---------------------------------------------------------------
# `cargo-chef` lets us cache the dependency build separately from the
# source build, so unchanged Cargo.toml/Cargo.lock means dependencies
# don't recompile.
FROM rust:1.89-bookworm AS chef
RUN cargo install cargo-chef --locked --version 0.1.71
WORKDIR /work

# ---- chef: prepare the recipe (deps-only manifest) --------------------------
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---- chef: build dependencies via the recipe --------------------------------
# Cache the cargo registry + git index + workspace target dir between
# builds. The `id=` keys scope the cache by purpose so a target-dir
# change doesn't bust the registry cache (and vice versa).
FROM chef AS deps-build
COPY --from=planner /work/recipe.json recipe.json
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=bezant-target,target=/work/target,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json -p bezant-server

# ---- build bezant-server ----------------------------------------------------
FROM deps-build AS rust-build
COPY . .
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=bezant-target,target=/work/target,sharing=locked \
    cargo build --release -p bezant-server \
    && cp /work/target/release/bezant-server /usr/local/bin/bezant-server

# ---- fetch + patch Gateway --------------------------------------------------
# Pinned to a separate stage so the (slow) IBKR download is cached
# independently of the source change cadence.
FROM debian:bookworm-slim AS gw-fetch
RUN apt-get update \
    && apt-get install -y --no-install-recommends curl unzip ca-certificates \
    && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL https://download2.interactivebrokers.com/portal/clientportal.gw.zip -o /tmp/gw.zip \
    && unzip -q /tmp/gw.zip -d /gw \
    && rm /tmp/gw.zip
# CPGateway's IPv4Validator expects octet-glob patterns like `192.*` — see
# docker/cpapi-gateway/Dockerfile for the history of why `*` / `.*` both fail.
RUN sed -i 's/- 192\.\*/- 10.*\n        - 100.*\n        - 172.*\n        - 192.*/' /gw/root/conf.yaml

# ---- runtime ----------------------------------------------------------------
FROM eclipse-temurin:17-jre

RUN apt-get update \
    && apt-get install -y --no-install-recommends tini ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

# Run as non-root (uid/gid 10001 — matches the convention from the
# standalone Dockerfile / distroless `nonroot` user). The Gateway,
# bezant-server, and entrypoint all bind only to high ports so no
# capabilities needed.
RUN groupadd -r -g 10001 bezant \
    && useradd -r -g bezant -u 10001 -s /usr/sbin/nologin bezant

WORKDIR /gw
COPY --from=gw-fetch --chown=bezant:bezant /gw/ /gw/
COPY --from=rust-build /usr/local/bin/bezant-server /usr/local/bin/bezant-server
COPY docker/combined/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh /usr/local/bin/bezant-server \
    && chown -R bezant:bezant /gw

USER bezant

ENV IBKR_GATEWAY_URL=https://127.0.0.1:5000/v1/api \
    BEZANT_BIND=0.0.0.0:8080 \
    RUST_LOG=bezant_server=info,bezant=info,tower_http=info

EXPOSE 8080
ENTRYPOINT ["/usr/bin/tini", "--", "/usr/local/bin/entrypoint.sh"]
