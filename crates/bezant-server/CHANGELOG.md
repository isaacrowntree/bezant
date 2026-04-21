# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/isaacrowntree/bezant/releases/tag/bezant-server-v0.1.0) - 2026-04-21

### Other

- log cookie-jar injections for debugging
- copy browser cookies into reqwest's jar on passthrough
- don't follow redirects, forward all response headers
- stop rewriting proxied Set-Cookie
- strip Secure flag from proxied Set-Cookie
- catch-all fallback proxies unmatched paths to the Gateway
- add order submit / cancel / list endpoints
- simplify CLI accounts, re-export url::Url, +12 tests
- enterprise polish — tests, CI, docs, OSS scaffolding
- Add bezant-server sidecar, Dockerfile, compose, facade tests
