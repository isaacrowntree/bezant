
# TradingLimits


## Properties

Name | Type
------------ | -------------
`dayQuantityLimits` | [Array&lt;DayQuantityLimit&gt;](DayQuantityLimit.md)
`efpQuantityLimits` | [EFPQuantityLimits](EFPQuantityLimits.md)
`orderQuantityLimits` | [Array&lt;OrderQuantityLimit&gt;](OrderQuantityLimit.md)
`orderValueLimits` | [OrderValueLimits](OrderValueLimits.md)

## Example

```typescript
import type { TradingLimits } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "dayQuantityLimits": null,
  "efpQuantityLimits": null,
  "orderQuantityLimits": null,
  "orderValueLimits": null,
} satisfies TradingLimits

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TradingLimits
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


