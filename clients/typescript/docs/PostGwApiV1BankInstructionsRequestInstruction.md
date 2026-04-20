
# PostGwApiV1BankInstructionsRequestInstruction


## Properties

Name | Type
------------ | -------------
`accountId` | string
`bankInstructionCode` | string
`bankInstructionName` | string
`clientInstructionId` | number
`creditAmount1` | number
`creditAmount2` | number
`pendingInstructionId` | number
`achType` | string
`clientAccountInfo` | [AchInstructionClientAccountInfo](AchInstructionClientAccountInfo.md)
`currency` | string
`bankInstructionMethod` | string
`financialInstitution` | [PredefinedDestinationInstructionFinancialInstitution](PredefinedDestinationInstructionFinancialInstitution.md)
`bankAccountNumber` | string
`bankBranchCode` | string
`bankClearingCode` | string
`debtorIdentificationDocumentType` | string

## Example

```typescript
import type { PostGwApiV1BankInstructionsRequestInstruction } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": U2323232,
  "bankInstructionCode": USACH,
  "bankInstructionName": Instruction Name,
  "clientInstructionId": 1012983,
  "creditAmount1": 1,
  "creditAmount2": 2,
  "pendingInstructionId": 35354345,
  "achType": DEBIT_CREDIT,
  "clientAccountInfo": null,
  "currency": CNH,
  "bankInstructionMethod": ACH,
  "financialInstitution": null,
  "bankAccountNumber": 132456,
  "bankBranchCode": 003,
  "bankClearingCode": 003,
  "debtorIdentificationDocumentType": hkId,
} satisfies PostGwApiV1BankInstructionsRequestInstruction

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PostGwApiV1BankInstructionsRequestInstruction
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


