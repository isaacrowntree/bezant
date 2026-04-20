
# SummaryOfAccountBalancesResponse


## Properties

Name | Type
------------ | -------------
`commodities` | [SummaryOfAccountBalancesResponseCommodities](SummaryOfAccountBalancesResponseCommodities.md)
`securities` | [SummaryOfAccountBalancesResponseSecurities](SummaryOfAccountBalancesResponseSecurities.md)
`total` | [SummaryOfAccountBalancesResponseTotal](SummaryOfAccountBalancesResponseTotal.md)

## Example

```typescript
import type { SummaryOfAccountBalancesResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "commodities": null,
  "securities": null,
  "total": null,
} satisfies SummaryOfAccountBalancesResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SummaryOfAccountBalancesResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


