
# PrimaryContributorType


## Properties

Name | Type
------------ | -------------
`address` | [Address](Address.md)
`employer` | string
`firstName` | string
`lastName` | string
`middleInitial` | string
`occupation` | string
`sourceOfFunds` | string
`suffix` | string

## Example

```typescript
import type { PrimaryContributorType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "address": null,
  "employer": null,
  "firstName": null,
  "lastName": null,
  "middleInitial": null,
  "occupation": null,
  "sourceOfFunds": null,
  "suffix": null,
} satisfies PrimaryContributorType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PrimaryContributorType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


