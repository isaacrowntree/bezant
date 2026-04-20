
# QueryBankInstructionResponse


## Properties

Name | Type
------------ | -------------
`instructionResult` | [QueryBankInstructionResult](QueryBankInstructionResult.md)
`instructionSetId` | number
`status` | number

## Example

```typescript
import type { QueryBankInstructionResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "instructionResult": null,
  "instructionSetId": -1988905739,
  "status": 202,
} satisfies QueryBankInstructionResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as QueryBankInstructionResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


