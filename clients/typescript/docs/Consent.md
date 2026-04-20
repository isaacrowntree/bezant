
# Consent


## Properties

Name | Type
------------ | -------------
`account_id` | string
`client_id` | string
`consent_id` | string
`consent_status` | string
`consented_at` | Date
`consented_scopes` | Set&lt;string&gt;
`user_id` | number

## Example

```typescript
import type { Consent } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "account_id": null,
  "client_id": null,
  "consent_id": null,
  "consent_status": null,
  "consented_at": null,
  "consented_scopes": null,
  "user_id": null,
} satisfies Consent

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Consent
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


