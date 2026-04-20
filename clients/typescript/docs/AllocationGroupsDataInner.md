
# AllocationGroupsDataInner


## Properties

Name | Type
------------ | -------------
`allocation_method` | [AllocationMethod](AllocationMethod.md)
`name` | string
`size` | number

## Example

```typescript
import type { AllocationGroupsDataInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "allocation_method": null,
  "name": null,
  "size": null,
} satisfies AllocationGroupsDataInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AllocationGroupsDataInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


