
# FormFileResponse


## Properties

Name | Type
------------ | -------------
`error` | [ErrorResponse](ErrorResponse.md)
`errorDescription` | string
`fileData` | [FileData](FileData.md)
`formDetails` | [Array&lt;FormDetails&gt;](FormDetails.md)
`hasError` | boolean
`timestamp` | Date

## Example

```typescript
import type { FormFileResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "error": null,
  "errorDescription": null,
  "fileData": null,
  "formDetails": null,
  "hasError": null,
  "timestamp": null,
} satisfies FormFileResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as FormFileResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


