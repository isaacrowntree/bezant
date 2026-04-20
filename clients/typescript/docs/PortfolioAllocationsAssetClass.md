
# PortfolioAllocationsAssetClass

Object containing values of positions sorted by long/short and asset class.

## Properties

Name | Type
------------ | -------------
`_long` | { [key: string]: number; }
`_short` | { [key: string]: number; }

## Example

```typescript
import type { PortfolioAllocationsAssetClass } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "_long": null,
  "_short": null,
} satisfies PortfolioAllocationsAssetClass

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PortfolioAllocationsAssetClass
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


