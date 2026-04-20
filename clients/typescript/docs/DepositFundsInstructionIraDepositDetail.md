
# DepositFundsInstructionIraDepositDetail


## Properties

Name | Type
------------ | -------------
`fromIraType` | string
`iraContributionType` | string
`iraTaxYearType` | string

## Example

```typescript
import type { DepositFundsInstructionIraDepositDetail } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "fromIraType": TRADITIONAL,
  "iraContributionType": ROLLOVER,
  "iraTaxYearType": CURRENT,
} satisfies DepositFundsInstructionIraDepositDetail

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as DepositFundsInstructionIraDepositDetail
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


