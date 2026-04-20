
# Au10TixDetailResponse


## Properties

Name | Type
------------ | -------------
`entityId` | number
`error` | [ErrorResponse](ErrorResponse.md)
`errorDescription` | string
`expiryDate` | Date
`externalId` | string
`hasError` | boolean
`startDate` | Date
`state` | string
`url` | string

## Example

```typescript
import type { Au10TixDetailResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "entityId": null,
  "error": null,
  "errorDescription": null,
  "expiryDate": null,
  "externalId": null,
  "hasError": null,
  "startDate": null,
  "state": null,
  "url": null,
} satisfies Au10TixDetailResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Au10TixDetailResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


