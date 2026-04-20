
# HighWaterMarkConfigurationType


## Properties

Name | Type
------------ | -------------
`numberOfPeriods` | number
`prorateForWithdrawals` | boolean

## Example

```typescript
import type { HighWaterMarkConfigurationType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "numberOfPeriods": null,
  "prorateForWithdrawals": null,
} satisfies HighWaterMarkConfigurationType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as HighWaterMarkConfigurationType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


