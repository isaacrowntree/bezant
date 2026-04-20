
# QueryBankInstructionResultAllOfBankInstructionDetails


## Properties

Name | Type
------------ | -------------
`bankAccountNumber` | string
`bankRoutingNumber` | string
`currency` | string
`instructionName` | string
`instructionStatus` | string
`type` | string

## Example

```typescript
import type { QueryBankInstructionResultAllOfBankInstructionDetails } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "bankAccountNumber": *****0000,
  "bankRoutingNumber": 000000000,
  "currency": USD,
  "instructionName": testinstr,
  "instructionStatus": PROCESSED,
  "type": CREDIT,
} satisfies QueryBankInstructionResultAllOfBankInstructionDetails

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as QueryBankInstructionResultAllOfBankInstructionDetails
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


