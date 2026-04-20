
# CreateUser


## Properties

Name | Type
------------ | -------------
`accountId` | string
`authorizedTrader` | boolean
`externalId` | string
`id` | string
`prefix` | string
`userName` | string

## Example

```typescript
import type { CreateUser } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": null,
  "authorizedTrader": null,
  "externalId": null,
  "id": null,
  "prefix": null,
  "userName": null,
} satisfies CreateUser

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as CreateUser
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


