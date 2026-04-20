
# TraditionalBankInstructionVerification


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

## Example

```typescript
import type { TraditionalBankInstructionVerification } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": U453454,
  "bankInstructionCode": ACHUS,
  "bankInstructionName": TestInstr,
  "clientInstructionId": 1012983,
  "creditAmount1": 1,
  "creditAmount2": 2,
  "pendingInstructionId": 35354345,
} satisfies TraditionalBankInstructionVerification

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TraditionalBankInstructionVerification
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


