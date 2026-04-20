
# GetContractSymbolsFromBodyRequest


## Properties

Name | Type
------------ | -------------
`fund` | boolean
`fundFamilyConidEx` | string
`more` | boolean
`name` | boolean
`pattern` | boolean
`referrer` | string
`secType` | string
`symbol` | string

## Example

```typescript
import type { GetContractSymbolsFromBodyRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "fund": null,
  "fundFamilyConidEx": null,
  "more": null,
  "name": null,
  "pattern": null,
  "referrer": null,
  "secType": null,
  "symbol": AAPL,
} satisfies GetContractSymbolsFromBodyRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GetContractSymbolsFromBodyRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


