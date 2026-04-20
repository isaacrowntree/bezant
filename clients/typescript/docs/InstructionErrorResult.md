
# InstructionErrorResult


## Properties

Name | Type
------------ | -------------
`clientInstructionId` | number
`description` | string
`ibReferenceId` | number
`instructionId` | number
`instructionStatus` | string
`instructionType` | string
`error` | [InstructionErrorResultAllOfError](InstructionErrorResultAllOfError.md)

## Example

```typescript
import type { InstructionErrorResult } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "clientInstructionId": 1012983,
  "description": Please poll for status after 10 minutes,
  "ibReferenceId": 23456745,
  "instructionId": 45123654,
  "instructionStatus": PENDING,
  "instructionType": INTERNAL_CASH_TRANSFER,
  "error": null,
} satisfies InstructionErrorResult

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as InstructionErrorResult
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


