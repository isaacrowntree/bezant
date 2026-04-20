
# AmRequestStatusResponse


## Properties

Name | Type
------------ | -------------
`acctId` | string
`message` | string
`requestId` | string
`requestType` | string
`status` | string

## Example

```typescript
import type { AmRequestStatusResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "acctId": null,
  "message": null,
  "requestId": null,
  "requestType": null,
  "status": null,
} satisfies AmRequestStatusResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AmRequestStatusResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


