
# StocksValueInner

Contains a series of objects for each symbol that matches the requested

## Properties

Name | Type
------------ | -------------
`assetClass` | string
`chineseName` | string
`contracts` | [Array&lt;StocksValueInnerContractsInner&gt;](StocksValueInnerContractsInner.md)
`name` | string

## Example

```typescript
import type { StocksValueInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "assetClass": null,
  "chineseName": null,
  "contracts": null,
  "name": null,
} satisfies StocksValueInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as StocksValueInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


