
# CounterpartyPermission


## Properties

Name | Type
------------ | -------------
`instruction_type_name` | string
`permission_name` | string
`permission_status` | string

## Example

```typescript
import type { CounterpartyPermission } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "instruction_type_name": null,
  "permission_name": null,
  "permission_status": null,
} satisfies CounterpartyPermission

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as CounterpartyPermission
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


