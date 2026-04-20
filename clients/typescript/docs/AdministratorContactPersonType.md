
# AdministratorContactPersonType


## Properties

Name | Type
------------ | -------------
`firstName` | string
`lastName` | string
`middleInitial` | string
`phoneNumber` | string
`suffix` | string

## Example

```typescript
import type { AdministratorContactPersonType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "firstName": null,
  "lastName": null,
  "middleInitial": null,
  "phoneNumber": null,
  "suffix": null,
} satisfies AdministratorContactPersonType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AdministratorContactPersonType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


