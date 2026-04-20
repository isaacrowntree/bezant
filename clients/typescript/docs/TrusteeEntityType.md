
# TrusteeEntityType


## Properties

Name | Type
------------ | -------------
`employees` | [Array&lt;Individual&gt;](Individual.md)
`legalEntity` | [LegalEntity](LegalEntity.md)

## Example

```typescript
import type { TrusteeEntityType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "employees": null,
  "legalEntity": null,
} satisfies TrusteeEntityType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TrusteeEntityType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


