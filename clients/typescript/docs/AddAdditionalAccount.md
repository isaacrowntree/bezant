
# AddAdditionalAccount


## Properties

Name | Type
------------ | -------------
`account` | [Account](Account.md)
`accountId` | string
`customer` | [Customer](Customer.md)
`documents` | [Array&lt;Document&gt;](Document.md)
`users` | [Array&lt;User&gt;](User.md)

## Example

```typescript
import type { AddAdditionalAccount } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "account": null,
  "accountId": null,
  "customer": null,
  "documents": null,
  "users": null,
} satisfies AddAdditionalAccount

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AddAdditionalAccount
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


