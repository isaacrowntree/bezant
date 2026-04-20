
# ProblemDetail


## Properties

Name | Type
------------ | -------------
`detail` | string
`invalidArguments` | [Array&lt;InvalidArgument&gt;](InvalidArgument.md)
`status` | number
`title` | string
`type` | string

## Example

```typescript
import type { ProblemDetail } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "detail": Missing required parameter: userName,
  "invalidArguments": null,
  "status": 400,
  "title": Bad Request,
  "type": /invalid-argument,
} satisfies ProblemDetail

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ProblemDetail
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


