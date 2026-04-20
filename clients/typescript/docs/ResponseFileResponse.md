
# ResponseFileResponse


## Properties

Name | Type
------------ | -------------
`data` | object
`error` | [ErrorResponse](ErrorResponse.md)
`errorDescription` | string
`hasError` | boolean
`isProcessed` | boolean
`name` | string

## Example

```typescript
import type { ResponseFileResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "data": null,
  "error": null,
  "errorDescription": null,
  "hasError": null,
  "isProcessed": null,
  "name": null,
} satisfies ResponseFileResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ResponseFileResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


