
# AssociationTypeEntities


## Properties

Name | Type
------------ | -------------
`individual` | [Array&lt;AssociatedIndividual&gt;](AssociatedIndividual.md)
`legalEntity` | [Array&lt;LegalEntity&gt;](LegalEntity.md)

## Example

```typescript
import type { AssociationTypeEntities } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "individual": null,
  "legalEntity": null,
} satisfies AssociationTypeEntities

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AssociationTypeEntities
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


