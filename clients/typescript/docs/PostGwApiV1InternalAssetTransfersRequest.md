
# PostGwApiV1InternalAssetTransfersRequest


## Properties

Name | Type
------------ | -------------
`instruction` | [InternalPositionTransferInstruction](InternalPositionTransferInstruction.md)
`instructionType` | string

## Example

```typescript
import type { PostGwApiV1InternalAssetTransfersRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "instruction": null,
  "instructionType": null,
} satisfies PostGwApiV1InternalAssetTransfersRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PostGwApiV1InternalAssetTransfersRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


