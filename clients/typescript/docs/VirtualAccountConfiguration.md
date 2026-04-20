
# VirtualAccountConfiguration


## Properties

Name | Type
------------ | -------------
`account_id_mappings` | [Set&lt;VirtualAccountIdMapping&gt;](VirtualAccountIdMapping.md)
`client_id` | string
`client_supports_virtual_accounts` | boolean
`user_id` | string

## Example

```typescript
import type { VirtualAccountConfiguration } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "account_id_mappings": null,
  "client_id": null,
  "client_supports_virtual_accounts": null,
  "user_id": null,
} satisfies VirtualAccountConfiguration

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as VirtualAccountConfiguration
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


