
# TickleResponse


## Properties

Name | Type
------------ | -------------
`collission` | boolean
`hmds` | [SuccessfulTickleResponseHmds](SuccessfulTickleResponseHmds.md)
`iserver` | [SuccessfulTickleResponseIserver](SuccessfulTickleResponseIserver.md)
`session` | string
`ssoExpires` | number
`userId` | number
`error` | string

## Example

```typescript
import type { TickleResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "collission": null,
  "hmds": null,
  "iserver": null,
  "session": null,
  "ssoExpires": null,
  "userId": null,
  "error": null,
} satisfies TickleResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TickleResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


