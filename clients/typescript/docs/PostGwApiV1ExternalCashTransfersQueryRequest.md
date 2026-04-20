
# PostGwApiV1ExternalCashTransfersQueryRequest


## Properties

Name | Type
------------ | -------------
`instruction` | [PostGwApiV1ExternalCashTransfersQueryRequestInstruction](PostGwApiV1ExternalCashTransfersQueryRequestInstruction.md)
`instructionType` | string

## Example

```typescript
import type { PostGwApiV1ExternalCashTransfersQueryRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "instruction": null,
  "instructionType": null,
} satisfies PostGwApiV1ExternalCashTransfersQueryRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PostGwApiV1ExternalCashTransfersQueryRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


