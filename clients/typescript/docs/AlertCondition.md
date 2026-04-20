
# AlertCondition


## Properties

Name | Type
------------ | -------------
`condition_logic_bind` | boolean
`condition_operator` | string
`condition_time_zone` | string
`condition_trigger_method` | number
`condition_type` | number
`condition_value` | string
`conidex` | string
`contract_description_1` | string

## Example

```typescript
import type { AlertCondition } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "condition_logic_bind": null,
  "condition_operator": null,
  "condition_time_zone": null,
  "condition_trigger_method": null,
  "condition_type": null,
  "condition_value": null,
  "conidex": null,
  "contract_description_1": null,
} satisfies AlertCondition

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AlertCondition
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


