
# AddEntity


## Properties

Name | Type
------------ | -------------
`addRelationships` | [Array&lt;AddRelationship&gt;](AddRelationship.md)
`documents` | [Array&lt;Document&gt;](Document.md)
`individual` | [Individual](Individual.md)
`legalEntity` | [LegalEntity](LegalEntity.md)

## Example

```typescript
import type { AddEntity } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "addRelationships": null,
  "documents": null,
  "individual": null,
  "legalEntity": null,
} satisfies AddEntity

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AddEntity
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


