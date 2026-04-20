
# UserEntity

Provide information about the particular entity

## Properties

Name | Type
------------ | -------------
`entityName` | string
`entityType` | string
`firstName` | string
`lastName` | string

## Example

```typescript
import type { UserEntity } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "entityName": null,
  "entityType": null,
  "firstName": null,
  "lastName": null,
} satisfies UserEntity

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as UserEntity
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


