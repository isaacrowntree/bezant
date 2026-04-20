
# DepositFundsPollingResultAllOfDepositDetails


## Properties

Name | Type
------------ | -------------
`amount` | number
`currency` | string
`openBanking` | [DepositFundsPollingResultAllOfDepositDetailsOpenBanking](DepositFundsPollingResultAllOfDepositDetailsOpenBanking.md)
`whenAvailable` | string

## Example

```typescript
import type { DepositFundsPollingResultAllOfDepositDetails } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "amount": null,
  "currency": null,
  "openBanking": null,
  "whenAvailable": null,
} satisfies DepositFundsPollingResultAllOfDepositDetails

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as DepositFundsPollingResultAllOfDepositDetails
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


