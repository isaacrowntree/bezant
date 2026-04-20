
# ClientConfiguration


## Properties

Name | Type
------------ | -------------
`accountSelectionMode` | string
`allowedIps` | Array&lt;string&gt;
`rateLimitConfigs` | [Array&lt;RateLimitConfig&gt;](RateLimitConfig.md)
`tokenRevocationEnabled` | boolean

## Example

```typescript
import type { ClientConfiguration } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountSelectionMode": null,
  "allowedIps": null,
  "rateLimitConfigs": null,
  "tokenRevocationEnabled": null,
} satisfies ClientConfiguration

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ClientConfiguration
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


