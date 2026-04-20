
# GetAccountsInModel200ResponseAccountInfoListInner


## Properties

Name | Type
------------ | -------------
`account` | string
`accountImbalance` | string
`alias` | string
`baseCcyAccount` | string
`costBasis` | string
`exchangeRate` | number
`nlv` | string
`numInstrumentsOutsideRange` | number
`unrealizedPnL` | string

## Example

```typescript
import type { GetAccountsInModel200ResponseAccountInfoListInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "account": null,
  "accountImbalance": null,
  "alias": null,
  "baseCcyAccount": null,
  "costBasis": null,
  "exchangeRate": null,
  "nlv": null,
  "numInstrumentsOutsideRange": null,
  "unrealizedPnL": null,
} satisfies GetAccountsInModel200ResponseAccountInfoListInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GetAccountsInModel200ResponseAccountInfoListInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


