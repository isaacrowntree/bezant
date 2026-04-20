# Security Policy

## Supported Versions

Only the latest `0.x` is receiving patches at this stage. Once we hit 1.0
we'll maintain the previous minor release alongside.

## Reporting a Vulnerability

**Please do not open a public GitHub issue for security vulnerabilities.**

Instead, email **isaac@triptech.com** with:

- A description of the vulnerability
- Steps to reproduce
- Any relevant versions (Rust toolchain, Gateway build, IBKR spec version)
- Your preferred disclosure timeline

You'll get an acknowledgement within **72 hours**. From there we'll triage,
produce a fix in a private branch, coordinate a disclosure date with you, and
ship a patched release with an advisory (CVE requested where appropriate).

## What counts

Bezant touches live brokerage accounts, so the blast radius of a bug can be
large. We treat the following as in-scope:

- Authentication / session handling bugs that could leak credentials or let a
  malicious caller hijack an authenticated Gateway
- Injection vectors (URL, header, JSON) that let a caller reach unintended
  endpoints on the Gateway
- Data-integrity bugs in the spec normaliser that could turn a read-only tool
  call into a write
- Any way to bypass the deliberate feature gating around order placement

Out of scope:

- Rate-limit bypasses against IBKR itself (report those to IBKR)
- TLS issues in self-signed mode (we document that this is intentional for
  local dev against the Gateway's default cert)

## Disclosure policy

Responsible disclosure gets full credit in the release notes and CVE. We do
not currently offer a paid bounty; if that changes the policy here will be
updated.
