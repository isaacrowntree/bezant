
# TaxPayerDetails


## Properties

Name | Type
------------ | -------------
`userName` | string
`w8Ben` | [FormW8BEN](FormW8BEN.md)

## Example

```typescript
import type { TaxPayerDetails } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "userName": null,
  "w8Ben": null,
} satisfies TaxPayerDetails

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TaxPayerDetails
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


