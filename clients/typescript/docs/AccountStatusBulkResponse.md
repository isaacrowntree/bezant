
# AccountStatusBulkResponse


## Properties

Name | Type
------------ | -------------
`accounts` | [Array&lt;AccountStatusResponse&gt;](AccountStatusResponse.md)
`limit` | number
`offset` | number
`total` | number

## Example

```typescript
import type { AccountStatusBulkResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accounts": null,
  "limit": null,
  "offset": null,
  "total": null,
} satisfies AccountStatusBulkResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AccountStatusBulkResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


