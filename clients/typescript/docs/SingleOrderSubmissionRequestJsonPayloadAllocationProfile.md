
# SingleOrderSubmissionRequestJsonPayloadAllocationProfile

The assigned contents for how the order should be allocated amongst subaccount(s).

## Properties

Name | Type
------------ | -------------
`alloc_type` | string
`allocations` | [Array&lt;SingleOrderSubmissionRequestJsonPayloadAllocationProfileAllocationsInner&gt;](SingleOrderSubmissionRequestJsonPayloadAllocationProfileAllocationsInner.md)

## Example

```typescript
import type { SingleOrderSubmissionRequestJsonPayloadAllocationProfile } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "alloc_type": null,
  "allocations": null,
} satisfies SingleOrderSubmissionRequestJsonPayloadAllocationProfile

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SingleOrderSubmissionRequestJsonPayloadAllocationProfile
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


