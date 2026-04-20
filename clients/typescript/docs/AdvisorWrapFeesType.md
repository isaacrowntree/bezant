
# AdvisorWrapFeesType


## Properties

Name | Type
------------ | -------------
`automatedFeesDetails` | [Array&lt;AutomatedWrapFeeDetailsType&gt;](AutomatedWrapFeeDetailsType.md)
`chargeAdvisor` | boolean
`chargeOtherFeesToAdvisor` | boolean
`highWaterMarkConfigHwma` | [HighWaterMarkType](HighWaterMarkType.md)
`highWaterMarkConfigHwmq` | [HighWaterMarkType](HighWaterMarkType.md)
`strategy` | string

## Example

```typescript
import type { AdvisorWrapFeesType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "automatedFeesDetails": null,
  "chargeAdvisor": null,
  "chargeOtherFeesToAdvisor": null,
  "highWaterMarkConfigHwma": null,
  "highWaterMarkConfigHwmq": null,
  "strategy": null,
} satisfies AdvisorWrapFeesType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AdvisorWrapFeesType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


