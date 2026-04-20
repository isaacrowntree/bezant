
# RecurringInstructionDetail


## Properties

Name | Type
------------ | -------------
`endDate` | string
`frequency` | string
`instructionName` | string
`startDate` | string

## Example

```typescript
import type { RecurringInstructionDetail } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "endDate": 2023-10-16,
  "frequency": MONTHLY,
  "instructionName": Arkansas-Test,
  "startDate": 2023-10-16,
} satisfies RecurringInstructionDetail

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as RecurringInstructionDetail
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


