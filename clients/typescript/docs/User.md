
# User


## Properties

Name | Type
------------ | -------------
`entity` | [UserEntity](UserEntity.md)
`hasRightCodeInd` | boolean
`roleId` | string
`userName` | string

## Example

```typescript
import type { User } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "entity": null,
  "hasRightCodeInd": null,
  "roleId": null,
  "userName": null,
} satisfies User

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as User
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


