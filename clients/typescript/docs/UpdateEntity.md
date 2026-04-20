
# UpdateEntity


## Properties

Name | Type
------------ | -------------
`addRelationships` | [Array&lt;AddRelationship&gt;](AddRelationship.md)
`deleteRelationships` | [Array&lt;DeleteRelationship&gt;](DeleteRelationship.md)
`documents` | [Array&lt;Document&gt;](Document.md)
`ibEntityId` | number
`individual` | [Individual](Individual.md)
`legalEntity` | [LegalEntity](LegalEntity.md)
`organization` | [Organization](Organization.md)
`trust` | [Trust](Trust.md)

## Example

```typescript
import type { UpdateEntity } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "addRelationships": null,
  "deleteRelationships": null,
  "documents": null,
  "ibEntityId": null,
  "individual": null,
  "legalEntity": null,
  "organization": null,
  "trust": null,
} satisfies UpdateEntity

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as UpdateEntity
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


