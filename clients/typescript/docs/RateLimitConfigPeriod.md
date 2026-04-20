
# RateLimitConfigPeriod


## Properties

Name | Type
------------ | -------------
`nano` | number
`negative` | boolean
`seconds` | number
`units` | [Array&lt;RateLimitConfigPeriodUnitsInner&gt;](RateLimitConfigPeriodUnitsInner.md)
`zero` | boolean

## Example

```typescript
import type { RateLimitConfigPeriod } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "nano": null,
  "negative": null,
  "seconds": null,
  "units": null,
  "zero": null,
} satisfies RateLimitConfigPeriod

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as RateLimitConfigPeriod
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


