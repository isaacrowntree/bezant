
# PostGwApiV1InstructionsCancelRequest


## Properties

Name | Type
------------ | -------------
`instruction` | [CancelInstruction](CancelInstruction.md)
`instructionType` | string

## Example

```typescript
import type { PostGwApiV1InstructionsCancelRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "instruction": null,
  "instructionType": null,
} satisfies PostGwApiV1InstructionsCancelRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PostGwApiV1InstructionsCancelRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


