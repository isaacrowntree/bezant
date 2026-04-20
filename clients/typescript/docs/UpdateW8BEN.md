
# UpdateW8BEN

Use UpdateTaxForm instead.

## Properties

Name | Type
------------ | -------------
`accountId` | string
`documents` | [Array&lt;Document&gt;](Document.md)
`inputLanguage` | string
`taxPayerDetails` | [TaxPayerDetails](TaxPayerDetails.md)
`translation` | boolean

## Example

```typescript
import type { UpdateW8BEN } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": null,
  "documents": null,
  "inputLanguage": null,
  "taxPayerDetails": null,
  "translation": null,
} satisfies UpdateW8BEN

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as UpdateW8BEN
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


