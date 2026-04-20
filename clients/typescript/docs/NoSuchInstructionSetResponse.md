
# NoSuchInstructionSetResponse


## Properties

Name | Type
------------ | -------------
`detail` | string
`instructionSetId` | number
`status` | number
`title` | string
`type` | string

## Example

```typescript
import type { NoSuchInstructionSetResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "detail": No such instruction set found,
  "instructionSetId": 8389943,
  "status": 404,
  "title": Not found,
  "type": /simple,
} satisfies NoSuchInstructionSetResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as NoSuchInstructionSetResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


