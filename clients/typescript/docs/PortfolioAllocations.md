
# PortfolioAllocations


## Properties

Name | Type
------------ | -------------
`assetClass` | [PortfolioAllocationsAssetClass](PortfolioAllocationsAssetClass.md)
`group` | [PortfolioAllocationsGroup](PortfolioAllocationsGroup.md)
`sector` | [PortfolioAllocationsSector](PortfolioAllocationsSector.md)

## Example

```typescript
import type { PortfolioAllocations } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "assetClass": null,
  "group": null,
  "sector": null,
} satisfies PortfolioAllocations

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PortfolioAllocations
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


