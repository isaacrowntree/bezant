
# Application


## Properties

Name | Type
------------ | -------------
`accounts` | [Array&lt;Account&gt;](Account.md)
`additionalAccounts` | [Array&lt;AddAdditionalAccount&gt;](AddAdditionalAccount.md)
`customer` | [Customer](Customer.md)
`documents` | [Array&lt;Document&gt;](Document.md)
`id` | string
`inputLanguage` | string
`masterAccountId` | string
`paperAccount` | boolean
`questionnaires` | [Questionnaires](Questionnaires.md)
`translation` | boolean
`users` | [Array&lt;User&gt;](User.md)

## Example

```typescript
import type { Application } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accounts": null,
  "additionalAccounts": null,
  "customer": null,
  "documents": null,
  "id": null,
  "inputLanguage": null,
  "masterAccountId": null,
  "paperAccount": null,
  "questionnaires": null,
  "translation": null,
  "users": null,
} satisfies Application

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Application
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


