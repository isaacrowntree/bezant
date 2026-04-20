
# TradingPermission


## Properties

Name | Type
------------ | -------------
`assetClass` | string
`country` | string
`exchangeGroup` | string
`product` | string

## Example

```typescript
import type { TradingPermission } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "assetClass": null,
  "country": null,
  "exchangeGroup": null,
  "product": null,
} satisfies TradingPermission

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TradingPermission
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


