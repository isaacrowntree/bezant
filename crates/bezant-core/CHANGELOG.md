# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/isaacrowntree/bezant/releases/tag/bezant-core-v0.1.0) - 2026-04-21

### Other

- set Content-Length: 0 header explicitly on empty POST
- map unauth 302s to NotAuthenticated in auth_status
- log raw auth_status response before typed parse
- force HTTP/1.1 so Akamai accepts empty POSTs
- don't follow redirects, forward all response headers
- simplify CLI accounts, re-export url::Url, +12 tests
- Add bezant-core crate README for crates.io
- enterprise polish — tests, CI, docs, OSS scaffolding
- Add snapshot tests driven by real IBKR example payloads
- WebSocket, pagination, symbol cache, tracing spans
- Add bezant-server sidecar, Dockerfile, compose, facade tests
- Initial commit: bezant-spec + bezant-api + bezant ergonomic facade
