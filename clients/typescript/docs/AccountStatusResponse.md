
# AccountStatusResponse


## Properties

Name | Type
------------ | -------------
`accountId` | string
`adminAccountId` | string
`dateClosed` | Date
`dateOpened` | Date
`dateStarted` | Date
`description` | string
`masterAccountId` | string
`message` | string
`state` | string
`status` | string

## Example

```typescript
import type { AccountStatusResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": null,
  "adminAccountId": null,
  "dateClosed": null,
  "dateOpened": null,
  "dateStarted": null,
  "description": null,
  "masterAccountId": null,
  "message": null,
  "state": null,
  "status": null,
} satisfies AccountStatusResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AccountStatusResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


