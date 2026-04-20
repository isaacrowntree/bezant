
# DepositFundsInstruction


## Properties

Name | Type
------------ | -------------
`accountId` | string
`amount` | number
`bankInstructionMethod` | string
`bankInstructionName` | string
`clientInstructionId` | number
`currency` | string
`identifier` | string
`iraDepositDetail` | [DepositFundsInstructionIraDepositDetail](DepositFundsInstructionIraDepositDetail.md)
`openBanking` | [DepositFundsInstructionOpenBanking](DepositFundsInstructionOpenBanking.md)
`recurringInstructionDetail` | [RecurringInstructionDetail](RecurringInstructionDetail.md)
`senderInstitutionName` | string
`sendingInstitution` | string
`specialInstruction` | string

## Example

```typescript
import type { DepositFundsInstruction } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": U46377,
  "amount": 100,
  "bankInstructionMethod": WIRE,
  "bankInstructionName": Instruction,
  "clientInstructionId": 1012983,
  "currency": USD,
  "identifier": identifier,
  "iraDepositDetail": null,
  "openBanking": null,
  "recurringInstructionDetail": null,
  "senderInstitutionName": Senders Institution name,
  "sendingInstitution": Sending Institution name,
  "specialInstruction": U46377,
} satisfies DepositFundsInstruction

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as DepositFundsInstruction
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


