
# RequiredFormsResponse


## Properties

Name | Type
------------ | -------------
`error` | [ErrorResponse](ErrorResponse.md)
`errorDescription` | string
`forms` | Array&lt;string&gt;
`hasError` | boolean

## Example

```typescript
import type { RequiredFormsResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "error": null,
  "errorDescription": null,
  "forms": null,
  "hasError": null,
} satisfies RequiredFormsResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as RequiredFormsResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


