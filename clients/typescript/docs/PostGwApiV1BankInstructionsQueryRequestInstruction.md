
# PostGwApiV1BankInstructionsQueryRequestInstruction


## Properties

Name | Type
------------ | -------------
`accountId` | string
`bankInstructionMethod` | string
`clientInstructionId` | number
`ibReferenceId` | number
`numberOfTransactions` | number

## Example

```typescript
import type { PostGwApiV1BankInstructionsQueryRequestInstruction } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": U399192,
  "bankInstructionMethod": WIRE,
  "clientInstructionId": 1012983,
  "ibReferenceId": -343872793,
  "numberOfTransactions": 15,
} satisfies PostGwApiV1BankInstructionsQueryRequestInstruction

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PostGwApiV1BankInstructionsQueryRequestInstruction
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


