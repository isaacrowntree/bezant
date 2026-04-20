
# GetAllModelPositionsRequest


## Properties

Name | Type
------------ | -------------
`limit` | number
`model` | string
`reqID` | number
`sortDirection` | string
`sortField` | string

## Example

```typescript
import type { GetAllModelPositionsRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "limit": 10,
  "model": Sample-Model,
  "reqID": 540609,
  "sortDirection": null,
  "sortField": null,
} satisfies GetAllModelPositionsRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GetAllModelPositionsRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


