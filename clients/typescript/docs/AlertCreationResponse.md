
# AlertCreationResponse


## Properties

Name | Type
------------ | -------------
`order_id` | number
`order_status` | string
`request_id` | number
`success` | boolean
`text` | string
`warning_message` | string

## Example

```typescript
import type { AlertCreationResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "order_id": null,
  "order_status": null,
  "request_id": null,
  "success": null,
  "text": null,
  "warning_message": null,
} satisfies AlertCreationResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AlertCreationResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


