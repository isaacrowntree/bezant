
# AssociatedEntity


## Properties

Name | Type
------------ | -------------
`AssociatedPersons` | [Array&lt;AssociatedPerson&gt;](AssociatedPerson.md)
`associations` | Set&lt;string&gt;
`email` | string
`entityId` | number
`externalCode` | string
`identityDocuments` | Array&lt;{ [key: string]: string; }&gt;
`mailing` | { [key: string]: string; }
`name` | string
`organizationCountry` | string
`phones` | { [key: string]: string; }
`residence` | { [key: string]: string; }
`taxTreatyDetails` | Array&lt;{ [key: string]: string; }&gt;

## Example

```typescript
import type { AssociatedEntity } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "AssociatedPersons": null,
  "associations": null,
  "email": null,
  "entityId": null,
  "externalCode": null,
  "identityDocuments": null,
  "mailing": null,
  "name": null,
  "organizationCountry": null,
  "phones": null,
  "residence": null,
  "taxTreatyDetails": null,
} satisfies AssociatedEntity

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AssociatedEntity
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


