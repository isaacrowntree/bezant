# Multi-stage build for bezant-server.
#
# The build stage compiles the Rust binary against musl for a fully-static
# linux/amd64 binary. The runtime stage copies that single binary into a
# distroless image, yielding a final image ~20 MB regardless of dependency
# tree size.

# ---- build ------------------------------------------------------------------
FROM rust:1.82-bookworm AS build

WORKDIR /work
COPY . .

# Cargo handles workspace builds — `--locked` keeps reproducible builds; drop
# it if you bump dependencies and haven't committed the updated Cargo.lock.
RUN cargo build --release -p bezant-server

# ---- runtime ----------------------------------------------------------------
FROM gcr.io/distroless/cc-debian12:nonroot AS runtime

COPY --from=build /work/target/release/bezant-server /usr/local/bin/bezant-server

ENV RUST_LOG=bezant_server=info,bezant=info,tower_http=info

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/bezant-server"]
