
# SummaryOfAccountBalancesResponseSecurities

Contains Security-specific balance details.

## Properties

Name | Type
------------ | -------------
`MTD_Interest` | string
`Pndng_Dbt_Crd_Chrgs` | string
`Prvs_Dy_Eqty_Wth_Ln_Vl` | string
`cash` | string
`equity_with_loan` | string
`net_liquidation` | string
`sec_gross_pos_val` | string

## Example

```typescript
import type { SummaryOfAccountBalancesResponseSecurities } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "MTD_Interest": null,
  "Pndng_Dbt_Crd_Chrgs": null,
  "Prvs_Dy_Eqty_Wth_Ln_Vl": null,
  "cash": null,
  "equity_with_loan": null,
  "net_liquidation": null,
  "sec_gross_pos_val": null,
} satisfies SummaryOfAccountBalancesResponseSecurities

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SummaryOfAccountBalancesResponseSecurities
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


