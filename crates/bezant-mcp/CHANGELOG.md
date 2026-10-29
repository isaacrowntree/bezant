# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.2](https://github.com/isaacrowntree/bezant/compare/bezant-mcp-v0.4.1...bezant-mcp-v0.4.2) - 2026-10-29

### Other

- update Cargo.lock dependencies

## [0.4.1](https://github.com/isaacrowntree/bezant/compare/bezant-mcp-v0.4.0...bezant-mcp-v0.4.1) - 2026-10-29

### Other

- updated the following local packages: bezant-api, bezant-core

## [0.4.0](https://github.com/isaacrowntree/bezant/compare/bezant-mcp-v0.3.0...bezant-mcp-v0.4.0) - 2026-10-17

### Breaking

- *(deps)* rmcp 1.6 -> 3.4 and schemars 0.8 -> 1. Both appear in the public
  API (`BezantMcp::tool_router`, the tool methods' `CallToolResult` /
  `McpError` return types, the `ServerHandler` impl and the
  `JsonSchema` derives on the argument structs), so library users must
  move to rmcp 3 / schemars 1 as well. The `bezant-mcp` binary is
  unaffected. ([#21](https://github.com/isaacrowntree/bezant/pull/21))

### Fixed

- *(deps)* rustls 0.23.46, clearing RUSTSEC-2026-0346 and related advisories ([#21](https://github.com/isaacrowntree/bezant/pull/21))
