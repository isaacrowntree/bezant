
# BusinessRejectResponse


## Properties

Name | Type
------------ | -------------
`instructionResult` | [InstructionErrorResult](InstructionErrorResult.md)
`instructionSetId` | number
`status` | number
`title` | string
`type` | string

## Example

```typescript
import type { BusinessRejectResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "instructionResult": null,
  "instructionSetId": 8389943,
  "status": 422,
  "title": Business Error,
  "type": /simple,
} satisfies BusinessRejectResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as BusinessRejectResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


