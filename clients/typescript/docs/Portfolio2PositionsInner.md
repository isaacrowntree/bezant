
# Portfolio2PositionsInner


## Properties

Name | Type
------------ | -------------
`assetClass` | string
`avgCost` | any
`avgPrice` | number
`conid` | number
`currency` | string
`description` | string
`group` | string
`isLastToLoq` | boolean
`marketPrice` | number
`marketValue` | number
`model` | string
`position` | number
`realizedPnl` | number
`secType` | string
`sector` | string
`timestamp` | number
`unrealizedPnl` | number

## Example

```typescript
import type { Portfolio2PositionsInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "assetClass": null,
  "avgCost": null,
  "avgPrice": null,
  "conid": null,
  "currency": null,
  "description": null,
  "group": null,
  "isLastToLoq": null,
  "marketPrice": null,
  "marketValue": null,
  "model": null,
  "position": null,
  "realizedPnl": null,
  "secType": null,
  "sector": null,
  "timestamp": null,
  "unrealizedPnl": null,
} satisfies Portfolio2PositionsInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Portfolio2PositionsInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


