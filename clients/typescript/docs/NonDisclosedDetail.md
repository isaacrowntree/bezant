
# NonDisclosedDetail


## Properties

Name | Type
------------ | -------------
`buyerSellBic` | string
`memberAccountId` | string
`psetBic` | string
`reagDeagBic` | string
`safeKeepingAccountId` | string
`settleDate` | string
`tradeDate` | string

## Example

```typescript
import type { NonDisclosedDetail } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "buyerSellBic": 320043,
  "memberAccountId": 123456,
  "psetBic": DTCYUS33XXX,
  "reagDeagBic": CH100164,
  "safeKeepingAccountId": 123456,
  "settleDate": 2023-07-18,
  "tradeDate": 2023-07-08,
} satisfies NonDisclosedDetail

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as NonDisclosedDetail
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


