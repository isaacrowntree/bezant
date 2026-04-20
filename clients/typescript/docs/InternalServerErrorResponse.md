
# InternalServerErrorResponse


## Properties

Name | Type
------------ | -------------
`status` | number
`title` | string
`type` | string

## Example

```typescript
import type { InternalServerErrorResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "status": 500,
  "title": Internal Server Error,
  "type": /simple,
} satisfies InternalServerErrorResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as InternalServerErrorResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


