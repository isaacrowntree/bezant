
# SummaryOfAccountBalancesResponseCommodities

Contains Commodity-specific balance details.

## Properties

Name | Type
------------ | -------------
`MTD_Interest` | string
`Pndng_Dbt_Crd_Chrgs` | string
`cash` | string
`equity_with_loan` | string
`net_liquidation` | string

## Example

```typescript
import type { SummaryOfAccountBalancesResponseCommodities } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "MTD_Interest": null,
  "Pndng_Dbt_Crd_Chrgs": null,
  "cash": null,
  "equity_with_loan": null,
  "net_liquidation": null,
} satisfies SummaryOfAccountBalancesResponseCommodities

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SummaryOfAccountBalancesResponseCommodities
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


