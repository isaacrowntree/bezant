
# AccountSupportType


## Properties

Name | Type
------------ | -------------
`acceptOwnersResideUS` | boolean
`administrator` | [AdministratorType](AdministratorType.md)
`administratorContactPerson` | [AdministratorContactPersonType](AdministratorContactPersonType.md)
`businessDescription` | string
`ownersResideUS` | boolean
`primaryContributor` | [PrimaryContributorType](PrimaryContributorType.md)
`solicitOwnersResideUS` | boolean
`type` | string

## Example

```typescript
import type { AccountSupportType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "acceptOwnersResideUS": null,
  "administrator": null,
  "administratorContactPerson": null,
  "businessDescription": null,
  "ownersResideUS": null,
  "primaryContributor": null,
  "solicitOwnersResideUS": null,
  "type": null,
} satisfies AccountSupportType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AccountSupportType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


