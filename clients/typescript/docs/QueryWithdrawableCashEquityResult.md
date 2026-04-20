
# QueryWithdrawableCashEquityResult


## Properties

Name | Type
------------ | -------------
`clientInstructionId` | number
`description` | string
`error` | [InstructionErrorResultAllOfError](InstructionErrorResultAllOfError.md)
`ibReferenceId` | number
`instructionId` | number
`instructionStatus` | string
`instructionType` | string
`accountId` | string
`availableEquityValue` | number
`currency` | string
`withdrawableCashAmount` | number

## Example

```typescript
import type { QueryWithdrawableCashEquityResult } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "clientInstructionId": 1012983,
  "description": Please poll for status after 10 minutes,
  "error": null,
  "ibReferenceId": 23456745,
  "instructionId": 45123654,
  "instructionStatus": PENDING,
  "instructionType": INTERNAL_CASH_TRANSFER,
  "accountId": null,
  "availableEquityValue": null,
  "currency": null,
  "withdrawableCashAmount": null,
} satisfies QueryWithdrawableCashEquityResult

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as QueryWithdrawableCashEquityResult
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


