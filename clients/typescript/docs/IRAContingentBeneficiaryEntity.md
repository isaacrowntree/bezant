
# IRAContingentBeneficiaryEntity


## Properties

Name | Type
------------ | -------------
`address` | [Address](Address.md)
`articleOfWill` | string
`entityType` | string
`executionDate` | Date
`executor` | [Individual](Individual.md)
`externalId` | string
`id` | string
`name` | string
`ownershipPercentage` | number
`relationship` | string
`title` | [Title](Title.md)

## Example

```typescript
import type { IRAContingentBeneficiaryEntity } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "address": null,
  "articleOfWill": null,
  "entityType": null,
  "executionDate": null,
  "executor": null,
  "externalId": null,
  "id": null,
  "name": null,
  "ownershipPercentage": null,
  "relationship": null,
  "title": null,
} satisfies IRAContingentBeneficiaryEntity

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as IRAContingentBeneficiaryEntity
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


