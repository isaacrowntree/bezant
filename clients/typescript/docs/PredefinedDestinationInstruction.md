
# PredefinedDestinationInstruction


## Properties

Name | Type
------------ | -------------
`accountId` | string
`bankInstructionMethod` | string
`bankInstructionName` | string
`clientInstructionId` | number
`currency` | string
`financialInstitution` | [PredefinedDestinationInstructionFinancialInstitution](PredefinedDestinationInstructionFinancialInstitution.md)

## Example

```typescript
import type { PredefinedDestinationInstruction } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": U2323232,
  "bankInstructionMethod": ACH,
  "bankInstructionName": Instruction,
  "clientInstructionId": 1012983,
  "currency": USD,
  "financialInstitution": null,
} satisfies PredefinedDestinationInstruction

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PredefinedDestinationInstruction
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


