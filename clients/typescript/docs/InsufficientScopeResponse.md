
# InsufficientScopeResponse


## Properties

Name | Type
------------ | -------------
`detail` | string
`status` | number
`title` | string
`type` | string

## Example

```typescript
import type { InsufficientScopeResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "detail": The access token fails to have sufficient scope,
  "status": 403,
  "title": Forbidden,
  "type": /simple,
} satisfies InsufficientScopeResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as InsufficientScopeResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


