
# RequestDetail


## Properties

Name | Type
------------ | -------------
`accountID` | string
`dateSubmitted` | string
`requestId` | number
`requestType` | string
`status` | string

## Example

```typescript
import type { RequestDetail } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountID": null,
  "dateSubmitted": null,
  "requestId": null,
  "requestType": null,
  "status": null,
} satisfies RequestDetail

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as RequestDetail
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


