
# AccountStatusRequest


## Properties

Name | Type
------------ | -------------
`endDate` | Date
`limit` | number
`offset` | number
`startDate` | Date
`status` | string

## Example

```typescript
import type { AccountStatusRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "endDate": null,
  "limit": null,
  "offset": null,
  "startDate": null,
  "status": null,
} satisfies AccountStatusRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AccountStatusRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


