# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/isaacrowntree/bezant/releases/tag/bezant-core-v0.1.0) - 2026-05-02

### Other

- send Origin/Referer matching the Gateway's own origin
- harden against control-char injection + mask values in Debug
- replace reqwest's path-aware Jar with NameKeyedJar
- 10-review pass — RFC compliance, error mapping, log hygiene, tests
- collapse multi-line expressions rustfmt flagged after the review pass
- manual Debug for Client (SymbolCache now derives Debug)
- review pass 2/2: address the 0.2-roadmap items in-line for 0.1
- centralise lint floor (forbid unsafe, warn missing docs)
- bump declared MSRV from 1.85 to 1.89 to match dependency reality
- rustfmt auth_status after the Akamai-401 refactor
- ship runnable examples for the three canonical flows
- declare package.metadata.docs.rs for every library crate
- copy browser cookies into reqwest's jar on passthrough
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
