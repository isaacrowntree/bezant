
# DepositFundsInstructionOpenBanking


## Properties

Name | Type
------------ | -------------
`plaidOptions` | [DepositFundsInstructionOpenBankingPlaidOptions](DepositFundsInstructionOpenBankingPlaidOptions.md)
`serviceProvider` | string

## Example

```typescript
import type { DepositFundsInstructionOpenBanking } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "plaidOptions": null,
  "serviceProvider": PLAID,
} satisfies DepositFundsInstructionOpenBanking

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as DepositFundsInstructionOpenBanking
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


