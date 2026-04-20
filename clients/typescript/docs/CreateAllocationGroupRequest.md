
# CreateAllocationGroupRequest


## Properties

Name | Type
------------ | -------------
`accounts` | [Array&lt;ModifyAllocationGroupRequestAccountsInner&gt;](ModifyAllocationGroupRequestAccountsInner.md)
`default_method` | [AllocationMethod](AllocationMethod.md)
`name` | string

## Example

```typescript
import type { CreateAllocationGroupRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accounts": null,
  "default_method": null,
  "name": Group_1_NetLiq,
} satisfies CreateAllocationGroupRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as CreateAllocationGroupRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


