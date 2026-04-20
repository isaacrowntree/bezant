
# ContractRulesTifDefaults

Object containing details about your TIF value defaults. These defaults can be viewed and modified in TWS\'s within the Global Configuration. 

## Properties

Name | Type
------------ | -------------
`DEFAULT_ACCT` | string
`PMALGO` | boolean
`SIZE` | string
`TIF` | string

## Example

```typescript
import type { ContractRulesTifDefaults } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "DEFAULT_ACCT": null,
  "PMALGO": null,
  "SIZE": null,
  "TIF": null,
} satisfies ContractRulesTifDefaults

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ContractRulesTifDefaults
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


