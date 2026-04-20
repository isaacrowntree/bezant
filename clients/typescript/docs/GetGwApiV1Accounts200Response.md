
# GetGwApiV1Accounts200Response


## Properties

Name | Type
------------ | -------------
`accountId` | string
`requestFileName` | string
`responseFileName` | string
`data` | object
`error` | [ErrorResponse](ErrorResponse.md)
`errorDescription` | string
`hasError` | boolean
`isProcessed` | boolean
`name` | string

## Example

```typescript
import type { GetGwApiV1Accounts200Response } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": null,
  "requestFileName": null,
  "responseFileName": null,
  "data": null,
  "error": null,
  "errorDescription": null,
  "hasError": null,
  "isProcessed": null,
  "name": null,
} satisfies GetGwApiV1Accounts200Response

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GetGwApiV1Accounts200Response
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


