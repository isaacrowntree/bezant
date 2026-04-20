
# RateLimitConfig


## Properties

Name | Type
------------ | -------------
`capacity` | number
`period` | [RateLimitConfigPeriod](RateLimitConfigPeriod.md)
`refillAmount` | number
`subject` | string

## Example

```typescript
import type { RateLimitConfig } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "capacity": null,
  "period": null,
  "refillAmount": null,
  "subject": null,
} satisfies RateLimitConfig

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as RateLimitConfig
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


