
# UserNameAvailableResponse


## Properties

Name | Type
------------ | -------------
`error` | [ErrorResponse](ErrorResponse.md)
`errorDescription` | string
`hasError` | boolean
`isAvailable` | boolean
`isValid` | boolean
`suggestedUserName` | Set&lt;string&gt;

## Example

```typescript
import type { UserNameAvailableResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "error": null,
  "errorDescription": null,
  "hasError": null,
  "isAvailable": null,
  "isValid": null,
  "suggestedUserName": null,
} satisfies UserNameAvailableResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as UserNameAvailableResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


