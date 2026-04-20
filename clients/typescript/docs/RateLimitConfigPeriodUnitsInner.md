
# RateLimitConfigPeriodUnitsInner


## Properties

Name | Type
------------ | -------------
`dateBased` | boolean
`duration` | [RateLimitConfigPeriodUnitsInnerDuration](RateLimitConfigPeriodUnitsInnerDuration.md)
`durationEstimated` | boolean
`timeBased` | boolean

## Example

```typescript
import type { RateLimitConfigPeriodUnitsInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "dateBased": null,
  "duration": null,
  "durationEstimated": null,
  "timeBased": null,
} satisfies RateLimitConfigPeriodUnitsInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as RateLimitConfigPeriodUnitsInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


