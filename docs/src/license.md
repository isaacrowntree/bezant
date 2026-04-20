# License

bezant is dual-licensed under your choice of:

- [MIT License](https://github.com/isaacrowntree/bezant/blob/main/LICENSE-MIT)
- [Apache License 2.0](https://github.com/isaacrowntree/bezant/blob/main/LICENSE-APACHE)

following the standard Rust ecosystem convention. Pick whichever works
better for your project; contributors agree their contributions are
available under the same terms.

## Third-party code

- The **vendored IBKR OpenAPI spec** under `crates/bezant-spec/` is
  Interactive Brokers' intellectual property. It's included here under
  fair-use conventions for API interoperability and is **not covered** by
  the Bezant license. If you redistribute Bezant, include the spec as-is
  and don't modify it outside of the documented normalisation pipeline.
- The **auto-generated Rust code** under `crates/bezant-api/src/generated/`
  is produced from the vendored spec by `oas3-gen`. We consider the
  generated code to be under Bezant's dual license; the IBKR-authored
  descriptions within rustdoc comments stay with IBKR.

## Not affiliated with Interactive Brokers

Bezant is an independent open-source project. Trading involves substantial
risk; this software is provided without warranty. See the license text.
