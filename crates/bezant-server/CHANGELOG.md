# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/isaacrowntree/bezant/releases/tag/bezant-server-v0.3.0) - 2026-05-03

### Other

- v0.3.0 — sharpening pass + version bump
- graceful shutdown + concurrency cap + reqwest pool tuning
- instrument hot paths + per-request correlation IDs
- promote Error::Other(String) sites into typed variants
- production hardening + crates.io readiness
- harden proxy + diagnostics for production deploys
- add /debug/probe to walk auth_status → ssodh/init → tickle → accounts
- harden against control-char injection + mask values in Debug
- replace reqwest's path-aware Jar with NameKeyedJar
- re-enable /debug/jar + INFO cookie-injection log for railway debugging
- path-conditional Origin/Referer rewrite — /sso/* keeps it, /v1/api/* swaps
- 10-review pass — RFC compliance, error mapping, log hygiene, tests
- collapse multi-line expressions rustfmt flagged after the review pass
- review pass 2/2: address the 0.2-roadmap items in-line for 0.1
- review pass: fix publish blockers + drop /debug/jar route
- centralise lint floor (forbid unsafe, warn missing docs)
- declare package.metadata.docs.rs for every library crate
- force Content-Length on proxied POST/PUT/DELETE
- /debug/jar endpoint for inspecting cookie jar
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
