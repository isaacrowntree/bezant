
# RegisteredClient


## Properties

Name | Type
------------ | -------------
`account_id` | string
`allowed_client_auth_methods` | Set&lt;string&gt;
`allowed_grant_types` | Set&lt;string&gt;
`client_category` | string
`client_id` | string
`client_name` | string
`client_secret_expires_at` | Date
`client_secret_issued_at` | Date
`client_secret_previous_expires_at` | Date
`client_status` | string
`client_type` | string
`client_uri` | string
`_configuration` | [ClientConfiguration](ClientConfiguration.md)
`created_at` | Date
`csid` | string
`description` | string
`id` | number
`jwks` | [ClientPublicKeySet](ClientPublicKeySet.md)
`logo_uri` | string
`policy_uri` | string
`redirect_uris` | Array&lt;string&gt;

## Example

```typescript
import type { RegisteredClient } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "account_id": null,
  "allowed_client_auth_methods": null,
  "allowed_grant_types": null,
  "client_category": null,
  "client_id": null,
  "client_name": null,
  "client_secret_expires_at": null,
  "client_secret_issued_at": null,
  "client_secret_previous_expires_at": null,
  "client_status": null,
  "client_type": null,
  "client_uri": null,
  "_configuration": null,
  "created_at": null,
  "csid": null,
  "description": null,
  "id": null,
  "jwks": null,
  "logo_uri": null,
  "policy_uri": null,
  "redirect_uris": null,
} satisfies RegisteredClient

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as RegisteredClient
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


