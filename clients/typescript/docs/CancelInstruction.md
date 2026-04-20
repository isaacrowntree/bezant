
# CancelInstruction


## Properties

Name | Type
------------ | -------------
`clientInstructionId` | number
`instructionId` | number
`reason` | string

## Example

```typescript
import type { CancelInstruction } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "clientInstructionId": 1012983,
  "instructionId": 43085477,
  "reason": Testing,
} satisfies CancelInstruction

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as CancelInstruction
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


