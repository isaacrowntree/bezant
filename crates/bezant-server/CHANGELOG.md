# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1](https://github.com/isaacrowntree/bezant/compare/bezant-server-v0.3.0...bezant-server-v0.3.1) - 2026-09-24

### Added

- *(health)* report "gateway up, api.ibkr.com failing" as its own code
- *(events)* POST /events/_reconnect, through the connector's command channel
- *(health)* /health/sso, readable in every state
- *(health)* report the SSO bridge, and heal it when it wedges

### Fixed

- *(events)* supervise the connector task; truncate on char boundaries
- *(events)* stop re-asking a subscription CPAPI honoured in silence
- *(events)* bump the epoch once per outage, on success; gaps only on `gap`
- *(events)* seed cursors and epoch from boot time; rings carry the live epoch
- *(events)* retry refused subscriptions, reconnect when the session rolls over
- *(ci)* clear cargo-deny advisories and pre-existing fmt/clippy/doc failures

### Other

- *(events)* describe the seeded cursors, live epoch and gap-topic-only gaps
- catch up README + CHANGELOG + ROADMAP + server docs to events surface
- wait for CPAPI server-ready frame before subscribing
- log every WS frame's topic at debug for diagnostics
- WS observability — capture ring + sqlite history + REST surface
- Revert "server: capture Set-Cookie from upstream into shared jar"
- capture Set-Cookie from upstream into shared jar
- cargo fmt drift + bump rustls past RUSTSEC-2026-0104
