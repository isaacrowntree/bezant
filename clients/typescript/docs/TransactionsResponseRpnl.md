
# TransactionsResponseRpnl

Returns the object containing the realized pnl for the contract on the date.

## Properties

Name | Type
------------ | -------------
`amt` | string
`data` | Array&lt;any&gt;
`items` | [TransactionsResponseRpnlItems](TransactionsResponseRpnlItems.md)

## Example

```typescript
import type { TransactionsResponseRpnl } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "amt": null,
  "data": null,
  "items": null,
} satisfies TransactionsResponseRpnl

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TransactionsResponseRpnl
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


