# bezant-client (TypeScript)

TypeScript/JavaScript client for the IBKR Client Portal Web API. Auto-generated
from the OpenAPI 3.1 spec vendored in [`crates/bezant-spec`](../../crates/bezant-spec/)
via [`openapi-generator-cli`](https://openapi-generator.tech/).

Same CPAPI endpoints as the Rust crates, but for Node / Deno / browser
consumers — so you don't need to run the [`bezant-server`](../../crates/bezant-server/)
sidecar if your application is already in JS land.

## Install (once published)

```sh
npm install bezant-client
```

Until it's on npm, depend on the tarball directly:

```sh
npm install github:isaacrowntree/bezant#main:clients/typescript
```

## Usage

```ts
import {
  Configuration,
  TradingAccountsApi,
  TradingPortfolioApi,
} from 'bezant-client';

const config = new Configuration({
  basePath: 'https://localhost:5000/v1/api',
  // The Gateway uses a self-signed cert by default. In Node, set
  // `NODE_TLS_REJECT_UNAUTHORIZED=0` in the environment (dev only!)
  // or supply a custom fetch with a permissive agent.
});

const accounts = await new TradingAccountsApi(config).getAllAccounts();
console.log(accounts);
```

## Regeneration

Run from the repository root:

```sh
./scripts/codegen-ts.sh
```

## License

Dual-licensed under MIT or Apache-2.0 at your option.
