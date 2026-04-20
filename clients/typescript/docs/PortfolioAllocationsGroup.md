
# PortfolioAllocationsGroup

Object containing values of positions sorted by long/short and Sector Group.

## Properties

Name | Type
------------ | -------------
`_long` | { [key: string]: number; }
`_short` | { [key: string]: number; }

## Example

```typescript
import type { PortfolioAllocationsGroup } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "_long": null,
  "_short": null,
} satisfies PortfolioAllocationsGroup

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PortfolioAllocationsGroup
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


