
# PartialStockPosition


## Properties

Name | Type
------------ | -------------
`all` | boolean
`exchange` | string
`numberOfShares` | number
`position` | string
`symbol` | string

## Example

```typescript
import type { PartialStockPosition } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "all": null,
  "exchange": null,
  "numberOfShares": null,
  "position": null,
  "symbol": null,
} satisfies PartialStockPosition

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PartialStockPosition
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


