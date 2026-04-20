
# WithdrawFundsInstruction


## Properties

Name | Type
------------ | -------------
`accountId` | string
`amount` | number
`bankInstructionMethod` | string
`bankInstructionName` | string
`clientInstructionId` | number
`currency` | string
`dateTimeToOccur` | Date
`iraWithdrawalDetail` | [WithdrawFundsInstructionIraWithdrawalDetail](WithdrawFundsInstructionIraWithdrawalDetail.md)
`recurringInstructionDetail` | [RecurringInstructionDetail](RecurringInstructionDetail.md)

## Example

```typescript
import type { WithdrawFundsInstruction } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": U46377,
  "amount": 100,
  "bankInstructionMethod": WIRE,
  "bankInstructionName": Instruction,
  "clientInstructionId": 1012983,
  "currency": USD,
  "dateTimeToOccur": null,
  "iraWithdrawalDetail": null,
  "recurringInstructionDetail": null,
} satisfies WithdrawFundsInstruction

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as WithdrawFundsInstruction
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


