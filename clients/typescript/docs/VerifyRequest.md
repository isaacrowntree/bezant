
# VerifyRequest


## Properties

Name | Type
------------ | -------------
`payload` | string
`requestId` | number
`userName` | string

## Example

```typescript
import type { VerifyRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "payload": null,
  "requestId": 127,
  "userName": hatty597,
} satisfies VerifyRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as VerifyRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


