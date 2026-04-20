# TypeScript client

Generated from the same vendored OpenAPI 3.1 spec as `bezant-api`, via
`openapi-generator-cli -g typescript-fetch`. Lives in
[`clients/typescript`](https://github.com/isaacrowntree/bezant/tree/main/clients/typescript).

## Install

Until it's on npm:

```sh
npm install github:isaacrowntree/bezant#main:clients/typescript
```

## Usage

```ts
import {
  Configuration,
  TradingAccountsApi,
  TradingPortfolioApi,
} from "bezant-client";

const config = new Configuration({
  basePath: "https://localhost:5000/v1/api",
});

const accounts = await new TradingAccountsApi(config).getAllAccounts();
const summary = await new TradingPortfolioApi(config).getPortfolioSummary({
  accountId: "DU123456",
});
```

## TLS gotcha

The Gateway ships a self-signed cert. Browsers reject it; Node / Deno
reject it by default.

- **Node (dev only!):** `NODE_TLS_REJECT_UNAUTHORIZED=0 npm run ...`
- **Production:** put Bezant behind a reverse proxy that terminates TLS
  with a trusted cert, or install the Gateway's cert into the system
  trust store.

## When to use this over `bezant-server`

- **TypeScript client** when you want typed methods / models in your
  frontend or Node app.
- **`bezant-server`** when you want language-agnostic REST, don't mind
  JSON-typed responses, or need the facade's features (keepalive,
  pagination) without reimplementing them in JS.
