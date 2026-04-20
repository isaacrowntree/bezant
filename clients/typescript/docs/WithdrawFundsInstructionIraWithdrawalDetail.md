
# WithdrawFundsInstructionIraWithdrawalDetail


## Properties

Name | Type
------------ | -------------
`fedIncomeTaxPercentage` | number
`iraWithholdType` | string
`stateCd` | string
`stateIncomeTaxPercentage` | number

## Example

```typescript
import type { WithdrawFundsInstructionIraWithdrawalDetail } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "fedIncomeTaxPercentage": 12,
  "iraWithholdType": NORMAL,
  "stateCd": TE,
  "stateIncomeTaxPercentage": 10,
} satisfies WithdrawFundsInstructionIraWithdrawalDetail

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as WithdrawFundsInstructionIraWithdrawalDetail
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


