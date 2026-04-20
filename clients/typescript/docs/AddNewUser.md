
# AddNewUser


## Properties

Name | Type
------------ | -------------
`accountId` | string
`inputLanguage` | string
`prefix` | string
`translation` | boolean
`userDetails` | [UserDetails](UserDetails.md)
`userName` | string

## Example

```typescript
import type { AddNewUser } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": null,
  "inputLanguage": null,
  "prefix": null,
  "translation": null,
  "userDetails": null,
  "userName": null,
} satisfies AddNewUser

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AddNewUser
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


