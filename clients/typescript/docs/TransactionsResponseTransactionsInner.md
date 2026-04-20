
# TransactionsResponseTransactionsInner


## Properties

Name | Type
------------ | -------------
`acctid` | string
`amt` | number
`conid` | number
`cur` | string
`date` | string
`desc` | string
`fxRate` | number
`pr` | number
`qty` | number
`type` | string

## Example

```typescript
import type { TransactionsResponseTransactionsInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "acctid": null,
  "amt": null,
  "conid": null,
  "cur": null,
  "date": null,
  "desc": null,
  "fxRate": null,
  "pr": null,
  "qty": null,
  "type": null,
} satisfies TransactionsResponseTransactionsInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TransactionsResponseTransactionsInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


