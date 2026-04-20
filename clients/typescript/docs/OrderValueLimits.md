
# OrderValueLimits


## Properties

Name | Type
------------ | -------------
`maxGrossValue` | number
`maxNetValue` | number
`maxOrderValue` | number
`netContractLimit` | number

## Example

```typescript
import type { OrderValueLimits } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "maxGrossValue": null,
  "maxNetValue": null,
  "maxOrderValue": null,
  "netContractLimit": null,
} satisfies OrderValueLimits

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as OrderValueLimits
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


