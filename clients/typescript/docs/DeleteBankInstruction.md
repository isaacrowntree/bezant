
# DeleteBankInstruction


## Properties

Name | Type
------------ | -------------
`accountId` | string
`bankInstructionMethod` | string
`bankInstructionName` | string
`clientInstructionId` | number
`currency` | string

## Example

```typescript
import type { DeleteBankInstruction } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": U399192,
  "bankInstructionMethod": WIRE,
  "bankInstructionName": Test-instruction,
  "clientInstructionId": 1012983,
  "currency": USD,
} satisfies DeleteBankInstruction

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as DeleteBankInstruction
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


