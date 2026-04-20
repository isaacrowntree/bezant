
# AccountSummaryResponseCashBalancesInner


## Properties

Name | Type
------------ | -------------
`balance` | number
`currency` | string
`settledCash` | number

## Example

```typescript
import type { AccountSummaryResponseCashBalancesInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "balance": null,
  "currency": null,
  "settledCash": null,
} satisfies AccountSummaryResponseCashBalancesInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AccountSummaryResponseCashBalancesInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


