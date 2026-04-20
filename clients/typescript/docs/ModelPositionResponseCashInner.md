
# ModelPositionResponseCashInner


## Properties

Name | Type
------------ | -------------
`actual` | number
`ccy` | string
`exchangeRate` | number
`instrumentImbalance` | number
`mv` | number
`target` | number

## Example

```typescript
import type { ModelPositionResponseCashInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "actual": null,
  "ccy": null,
  "exchangeRate": null,
  "instrumentImbalance": null,
  "mv": null,
  "target": null,
} satisfies ModelPositionResponseCashInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ModelPositionResponseCashInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


