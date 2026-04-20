
# AlertDeletionResponse


## Properties

Name | Type
------------ | -------------
`failure_list` | string
`order_id` | number
`request_id` | number
`success` | boolean
`text` | string

## Example

```typescript
import type { AlertDeletionResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "failure_list": null,
  "order_id": null,
  "request_id": null,
  "success": null,
  "text": null,
} satisfies AlertDeletionResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AlertDeletionResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


