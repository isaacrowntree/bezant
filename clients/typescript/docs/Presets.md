
# Presets


## Properties

Name | Type
------------ | -------------
`default_method_for_all` | string
`group_auto_close_positions` | boolean
`group_proportional_allocation` | boolean
`profiles_auto_close_positions` | boolean
`strict_credit_check` | boolean

## Example

```typescript
import type { Presets } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "default_method_for_all": null,
  "group_auto_close_positions": null,
  "group_proportional_allocation": null,
  "profiles_auto_close_positions": null,
  "strict_credit_check": null,
} satisfies Presets

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Presets
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


