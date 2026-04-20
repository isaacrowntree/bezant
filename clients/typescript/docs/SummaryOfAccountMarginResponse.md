
# SummaryOfAccountMarginResponse


## Properties

Name | Type
------------ | -------------
`Crypto_at_Paxos` | [SummaryOfAccountMarginResponseCryptoAtPaxos](SummaryOfAccountMarginResponseCryptoAtPaxos.md)
`commodities` | [SummaryOfAccountMarginResponseCommodities](SummaryOfAccountMarginResponseCommodities.md)
`securities` | [SummaryOfAccountMarginResponseSecurities](SummaryOfAccountMarginResponseSecurities.md)
`total` | [SummaryOfAccountMarginResponseCryptoAtPaxos](SummaryOfAccountMarginResponseCryptoAtPaxos.md)

## Example

```typescript
import type { SummaryOfAccountMarginResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "Crypto_at_Paxos": null,
  "commodities": null,
  "securities": null,
  "total": null,
} satisfies SummaryOfAccountMarginResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SummaryOfAccountMarginResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


