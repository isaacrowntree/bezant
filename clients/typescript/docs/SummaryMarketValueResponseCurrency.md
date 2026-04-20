
# SummaryMarketValueResponseCurrency

Returns an object containing market value details of the currency and positions held using that currency.

## Properties

Name | Type
------------ | -------------
`Cryptocurrency` | string
`Exchange_Rate` | string
`Govt_Bonds` | string
`MTD_Interest` | string
`Notional_CFD` | string
`bonds` | string
`cfd` | string
`commodity` | string
`dividends_receivable` | string
`funds` | string
`future_options` | string
`futures` | string
`issuer_option` | string
`money_market` | string
`mutual_funds` | string
`net_liquidation` | string
`options` | string
`realized_pnl` | string
`settled_cash` | string
`stock` | string
`t_bills` | string
`total_cash` | string
`unrealized_pnl` | string
`warrants` | string

## Example

```typescript
import type { SummaryMarketValueResponseCurrency } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "Cryptocurrency": null,
  "Exchange_Rate": null,
  "Govt_Bonds": null,
  "MTD_Interest": null,
  "Notional_CFD": null,
  "bonds": null,
  "cfd": null,
  "commodity": null,
  "dividends_receivable": null,
  "funds": null,
  "future_options": null,
  "futures": null,
  "issuer_option": null,
  "money_market": null,
  "mutual_funds": null,
  "net_liquidation": null,
  "options": null,
  "realized_pnl": null,
  "settled_cash": null,
  "stock": null,
  "t_bills": null,
  "total_cash": null,
  "unrealized_pnl": null,
  "warrants": null,
} satisfies SummaryMarketValueResponseCurrency

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SummaryMarketValueResponseCurrency
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


