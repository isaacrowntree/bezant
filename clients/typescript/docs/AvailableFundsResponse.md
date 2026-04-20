
# AvailableFundsResponse

Contains a combined overview of Commidity, Security and Crypto fund values.

## Properties

Name | Type
------------ | -------------
`Crypto_at_Paxos` | [Funds](Funds.md)
`commodities` | [Funds](Funds.md)
`securities` | [AvailableFundsResponseSecurities](AvailableFundsResponseSecurities.md)
`total` | [AvailableFundsResponseTotal](AvailableFundsResponseTotal.md)

## Example

```typescript
import type { AvailableFundsResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "Crypto_at_Paxos": null,
  "commodities": null,
  "securities": null,
  "total": null,
} satisfies AvailableFundsResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AvailableFundsResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


