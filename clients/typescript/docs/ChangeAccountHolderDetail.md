
# ChangeAccountHolderDetail


## Properties

Name | Type
------------ | -------------
`accountId` | string
`documents` | [DocumentSubmission](DocumentSubmission.md)
`inputLanguage` | string
`newAccountHolderDetails` | [Array&lt;AssociatedIndividual&gt;](AssociatedIndividual.md)
`referenceUserName` | string
`translation` | boolean

## Example

```typescript
import type { ChangeAccountHolderDetail } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": null,
  "documents": null,
  "inputLanguage": null,
  "newAccountHolderDetails": null,
  "referenceUserName": null,
  "translation": null,
} satisfies ChangeAccountHolderDetail

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ChangeAccountHolderDetail
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


