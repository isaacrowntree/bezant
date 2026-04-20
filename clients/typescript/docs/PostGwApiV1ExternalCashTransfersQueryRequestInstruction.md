
# PostGwApiV1ExternalCashTransfersQueryRequestInstruction


## Properties

Name | Type
------------ | -------------
`accountId` | string
`bankAccountNumber` | string
`bankInstructionName` | string
`bankRoutingNumber` | string
`clientInstructionId` | number
`currency` | string
`year` | string

## Example

```typescript
import type { PostGwApiV1ExternalCashTransfersQueryRequestInstruction } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": U399192,
  "bankAccountNumber": 9876543210,
  "bankInstructionName": test,
  "bankRoutingNumber": 122199983,
  "clientInstructionId": 1012983,
  "currency": USD,
  "year": 2003,
} satisfies PostGwApiV1ExternalCashTransfersQueryRequestInstruction

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PostGwApiV1ExternalCashTransfersQueryRequestInstruction
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


