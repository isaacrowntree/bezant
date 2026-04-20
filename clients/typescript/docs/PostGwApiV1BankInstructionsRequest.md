
# PostGwApiV1BankInstructionsRequest


## Properties

Name | Type
------------ | -------------
`instruction` | [PostGwApiV1BankInstructionsRequestInstruction](PostGwApiV1BankInstructionsRequestInstruction.md)
`instructionType` | string

## Example

```typescript
import type { PostGwApiV1BankInstructionsRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "instruction": null,
  "instructionType": null,
} satisfies PostGwApiV1BankInstructionsRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PostGwApiV1BankInstructionsRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


