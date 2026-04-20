
# EddaInstruction


## Properties

Name | Type
------------ | -------------
`accountId` | string
`bankAccountNumber` | string
`bankBranchCode` | string
`bankClearingCode` | string
`bankInstructionName` | string
`clientInstructionId` | number
`currency` | string
`debtorIdentificationDocumentType` | string

## Example

```typescript
import type { EddaInstruction } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": U2323232,
  "bankAccountNumber": 132456,
  "bankBranchCode": 003,
  "bankClearingCode": 003,
  "bankInstructionName": Instruction Name,
  "clientInstructionId": 1012983,
  "currency": CNH,
  "debtorIdentificationDocumentType": hkId,
} satisfies EddaInstruction

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as EddaInstruction
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


