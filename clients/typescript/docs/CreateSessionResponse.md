
# CreateSessionResponse


## Properties

Name | Type
------------ | -------------
`access_token` | string
`active` | boolean
`token_type` | string

## Example

```typescript
import type { CreateSessionResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "access_token": eyJ0eXAiOiJKV1...,
  "active": null,
  "token_type": Bearer,
} satisfies CreateSessionResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as CreateSessionResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


