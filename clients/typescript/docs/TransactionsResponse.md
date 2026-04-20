
# TransactionsResponse


## Properties

Name | Type
------------ | -------------
`currency` | string
`from` | number
`id` | string
`includesRealTime` | boolean
`nd` | number
`rc` | number
`rpnl` | [TransactionsResponseRpnl](TransactionsResponseRpnl.md)
`to` | number
`transactions` | [Array&lt;TransactionsResponseTransactionsInner&gt;](TransactionsResponseTransactionsInner.md)

## Example

```typescript
import type { TransactionsResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "currency": null,
  "from": null,
  "id": null,
  "includesRealTime": null,
  "nd": null,
  "rc": null,
  "rpnl": null,
  "to": null,
  "transactions": null,
} satisfies TransactionsResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TransactionsResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


