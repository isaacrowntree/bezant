
# AutomatedWrapFeeDetailsType


## Properties

Name | Type
------------ | -------------
`annualBlendedPercentages` | [Array&lt;AnnualBlendedPercentage&gt;](AnnualBlendedPercentage.md)
`maxFee` | number
`navRanges` | [Array&lt;NAVRangeType&gt;](NAVRangeType.md)
`numContracts` | number
`perTradeMarkups` | [CommissionScheduleType](CommissionScheduleType.md)
`percentOfNLVCap` | string
`percentOfNLVCapQ` | string
`postFrequency` | string
`type` | string

## Example

```typescript
import type { AutomatedWrapFeeDetailsType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "annualBlendedPercentages": null,
  "maxFee": null,
  "navRanges": null,
  "numContracts": null,
  "perTradeMarkups": null,
  "percentOfNLVCap": null,
  "percentOfNLVCapQ": null,
  "postFrequency": null,
  "type": null,
} satisfies AutomatedWrapFeeDetailsType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AutomatedWrapFeeDetailsType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


