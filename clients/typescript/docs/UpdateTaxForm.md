
# UpdateTaxForm


## Properties

Name | Type
------------ | -------------
`accountId` | string
`documents` | [Array&lt;Document&gt;](Document.md)
`entityId` | string
`externalId` | string
`inputLanguage` | string
`localTaxForms` | [Array&lt;LocalTaxForm&gt;](LocalTaxForm.md)
`translation` | boolean
`w8Ben` | [FormW8BEN](FormW8BEN.md)
`w8BenE` | [FormW8BENE](FormW8BENE.md)
`w9` | [FormW9](FormW9.md)

## Example

```typescript
import type { UpdateTaxForm } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": null,
  "documents": null,
  "entityId": null,
  "externalId": null,
  "inputLanguage": null,
  "localTaxForms": null,
  "translation": null,
  "w8Ben": null,
  "w8BenE": null,
  "w9": null,
} satisfies UpdateTaxForm

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as UpdateTaxForm
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


