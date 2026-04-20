
# QueryWithdrawableCashEquity


## Properties

Name | Type
------------ | -------------
`accountId` | string
`clientInstructionId` | number
`currency` | string

## Example

```typescript
import type { QueryWithdrawableCashEquity } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": U399192,
  "clientInstructionId": 1012983,
  "currency": USD,
} satisfies QueryWithdrawableCashEquity

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as QueryWithdrawableCashEquity
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


