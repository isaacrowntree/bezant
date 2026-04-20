
# EchoResponse


## Properties

Name | Type
------------ | -------------
`_queryParameters` | object
`requestMethod` | string
`securityPolicy` | string

## Example

```typescript
import type { EchoResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "_queryParameters": {},
  "requestMethod": GET,
  "securityPolicy": HTTPS,
} satisfies EchoResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as EchoResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


