
# NoSuchInstructionResponse


## Properties

Name | Type
------------ | -------------
`detail` | string
`status` | number
`title` | string
`type` | string

## Example

```typescript
import type { NoSuchInstructionResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "detail": No such instruction found,
  "status": 404,
  "title": Not found,
  "type": /simple,
} satisfies NoSuchInstructionResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as NoSuchInstructionResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


