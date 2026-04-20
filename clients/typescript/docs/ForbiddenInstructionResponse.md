
# ForbiddenInstructionResponse


## Properties

Name | Type
------------ | -------------
`detail` | string
`instructionResult` | [InstructionErrorResult](InstructionErrorResult.md)
`instructionSetId` | number
`status` | number
`title` | string
`type` | string

## Example

```typescript
import type { ForbiddenInstructionResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "detail": Input is not a JSON Object or doesn't contain all expected fields,
  "instructionResult": null,
  "instructionSetId": 8389943,
  "status": 400,
  "title": Bad Request,
  "type": /invalid-argument,
} satisfies ForbiddenInstructionResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ForbiddenInstructionResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


