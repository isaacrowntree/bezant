
# EnumerationResponse


## Properties

Name | Type
------------ | -------------
`enumerationsType` | string
`formNumber` | string
`jsonData` | object

## Example

```typescript
import type { EnumerationResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "enumerationsType": null,
  "formNumber": null,
  "jsonData": null,
} satisfies EnumerationResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as EnumerationResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


