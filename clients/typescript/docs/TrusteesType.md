
# TrusteesType


## Properties

Name | Type
------------ | -------------
`entities` | [Array&lt;TrusteeEntityType&gt;](TrusteeEntityType.md)
`individuals` | [Array&lt;TrusteeIndividual&gt;](TrusteeIndividual.md)

## Example

```typescript
import type { TrusteesType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "entities": null,
  "individuals": null,
} satisfies TrusteesType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TrusteesType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


