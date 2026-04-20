
# AchInstruction


## Properties

Name | Type
------------ | -------------
`accountId` | string
`achType` | string
`bankInstructionCode` | string
`bankInstructionName` | string
`clientAccountInfo` | [AchInstructionClientAccountInfo](AchInstructionClientAccountInfo.md)
`clientInstructionId` | number
`currency` | string

## Example

```typescript
import type { AchInstruction } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": U223454,
  "achType": DEBIT_CREDIT,
  "bankInstructionCode": USACH,
  "bankInstructionName": TestInstr,
  "clientAccountInfo": null,
  "clientInstructionId": 1012983,
  "currency": USD,
} satisfies AchInstruction

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AchInstruction
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


