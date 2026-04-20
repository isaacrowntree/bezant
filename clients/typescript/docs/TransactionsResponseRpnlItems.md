
# TransactionsResponseRpnlItems


## Properties

Name | Type
------------ | -------------
`acctid` | string
`amt` | string
`conid` | string
`cur` | string
`date` | string
`fxRate` | number
`side` | string

## Example

```typescript
import type { TransactionsResponseRpnlItems } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "acctid": null,
  "amt": null,
  "conid": null,
  "cur": null,
  "date": null,
  "fxRate": null,
  "side": null,
} satisfies TransactionsResponseRpnlItems

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TransactionsResponseRpnlItems
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


