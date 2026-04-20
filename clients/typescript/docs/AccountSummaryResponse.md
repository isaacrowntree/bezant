
# AccountSummaryResponse

Successful return contianing an array of at-a-glance account details.

## Properties

Name | Type
------------ | -------------
`SMA` | number
`accountType` | string
`accruedInterest` | number
`availableFunds` | number
`balance` | number
`buyingPower` | number
`cashBalances` | [Array&lt;AccountSummaryResponseCashBalancesInner&gt;](AccountSummaryResponseCashBalancesInner.md)
`equityWithLoanValue` | number
`excessLiquidity` | number
`initialMargin` | number
`maintenanceMargin` | number
`netLiquidationValue` | number
`regTLoan` | number
`regTMargin` | number
`securitiesGVP` | number
`status` | string
`totalCashValue` | number

## Example

```typescript
import type { AccountSummaryResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "SMA": null,
  "accountType": null,
  "accruedInterest": null,
  "availableFunds": null,
  "balance": null,
  "buyingPower": null,
  "cashBalances": null,
  "equityWithLoanValue": null,
  "excessLiquidity": null,
  "initialMargin": null,
  "maintenanceMargin": null,
  "netLiquidationValue": null,
  "regTLoan": null,
  "regTMargin": null,
  "securitiesGVP": null,
  "status": null,
  "totalCashValue": null,
} satisfies AccountSummaryResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AccountSummaryResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


