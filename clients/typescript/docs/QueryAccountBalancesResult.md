
# QueryAccountBalancesResult


## Properties

Name | Type
------------ | -------------
`clientInstructionId` | number
`description` | string
`ibReferenceId` | number
`instructionId` | number
`instructionStatus` | string
`instructionType` | string
`accountId` | string
`clientBankAccountWithdrawableBalance` | Array&lt;any&gt;
`currency` | string
`transferableAmount` | number
`withdrawableAmount` | number

## Example

```typescript
import type { QueryAccountBalancesResult } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "clientInstructionId": 1012983,
  "description": Please poll for status after 10 minutes,
  "ibReferenceId": 23456745,
  "instructionId": 45123654,
  "instructionStatus": PENDING,
  "instructionType": INTERNAL_CASH_TRANSFER,
  "accountId": U4567,
  "clientBankAccountWithdrawableBalance": null,
  "currency": USD,
  "transferableAmount": 1234,
  "withdrawableAmount": 1234,
} satisfies QueryAccountBalancesResult

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as QueryAccountBalancesResult
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


