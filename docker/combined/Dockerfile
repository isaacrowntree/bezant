# Combined cpapi-gateway + bezant-server image.
#
# For single-service deploys (Railway, fly.io, a bare VM) where both
# processes always live together: the IBKR Client Portal Gateway on
# 127.0.0.1:5000 and bezant-server fronting it on :8080. The user hits
# bezant-server for both the interactive login UI (via fallback
# passthrough) and the JSON API — the cookie lands in bezant-server's
# reqwest jar, which is what authenticated API calls use.
#
# Consumers who want to run the two separately can still use the
# standalone images in ../cpapi-gateway/ and the repo-root Dockerfile.

# ---- build bezant-server ----------------------------------------------------
FROM rust:1.89-bookworm AS rust-build
WORKDIR /work
COPY . .
RUN cargo build --release -p bezant-server

# ---- fetch + patch Gateway --------------------------------------------------
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

WORKDIR /gw
COPY --from=gw-fetch /gw/ /gw/
COPY --from=rust-build /work/target/release/bezant-server /usr/local/bin/bezant-server
COPY docker/combined/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

ENV IBKR_GATEWAY_URL=https://127.0.0.1:5000/v1/api \
    BEZANT_BIND=0.0.0.0:8080 \
    RUST_LOG=bezant_server=info,bezant=info,tower_http=info

EXPOSE 8080
ENTRYPOINT ["/usr/bin/tini", "--", "/usr/local/bin/entrypoint.sh"]
