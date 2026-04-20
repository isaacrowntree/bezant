
# IndividualComboPosition


## Properties

Name | Type
------------ | -------------
`acctId` | string
`assetClass` | string
`avgCost` | number
`avgPrice` | number
`conExchMap` | Array&lt;any&gt;
`conid` | number
`contractDesc` | string
`currency` | string
`exchs` | object
`exerciseStyle` | string
`expiry` | string
`mktPrice` | number
`mktValue` | number
`multiplier` | number
`position` | number
`putOrCall` | string
`realizedPnl` | number
`strike` | string
`undConid` | number
`unrealizedPnl` | number

## Example

```typescript
import type { IndividualComboPosition } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "acctId": null,
  "assetClass": null,
  "avgCost": null,
  "avgPrice": null,
  "conExchMap": null,
  "conid": null,
  "contractDesc": null,
  "currency": null,
  "exchs": null,
  "exerciseStyle": null,
  "expiry": null,
  "mktPrice": null,
  "mktValue": null,
  "multiplier": null,
  "position": null,
  "putOrCall": null,
  "realizedPnl": null,
  "strike": null,
  "undConid": null,
  "unrealizedPnl": null,
} satisfies IndividualComboPosition

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as IndividualComboPosition
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


