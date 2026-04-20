
# ContractRulesOrderDefaults

Indicates default order type for the given security type.

## Properties

Name | Type
------------ | -------------
`LMT` | [ContractRulesOrderDefaultsLMT](ContractRulesOrderDefaultsLMT.md)

## Example

```typescript
import type { ContractRulesOrderDefaults } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "LMT": null,
} satisfies ContractRulesOrderDefaults

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ContractRulesOrderDefaults
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


