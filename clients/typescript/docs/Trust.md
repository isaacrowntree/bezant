
# Trust


## Properties

Name | Type
------------ | -------------
`identification` | [TrustIdentification](TrustIdentification.md)
`regulatoryInformation` | [RegulatoryInformation](RegulatoryInformation.md)

## Example

```typescript
import type { Trust } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "identification": null,
  "regulatoryInformation": null,
} satisfies Trust

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Trust
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


