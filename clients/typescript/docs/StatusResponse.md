
# StatusResponse


## Properties

Name | Type
------------ | -------------
`dateSubmitted` | Date
`fileData` | [FileData](FileData.md)
`requestId` | number

## Example

```typescript
import type { StatusResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "dateSubmitted": null,
  "fileData": null,
  "requestId": null,
} satisfies StatusResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as StatusResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


