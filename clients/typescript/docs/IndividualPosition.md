
# IndividualPosition

A specific account\'s position in the requested conid.

## Properties

Name | Type
------------ | -------------
`acctId` | string
`allExchanges` | string
`assetClass` | string
`avgCost` | number
`avgPrice` | number
`baseAvgCost` | number
`baseAvgPrice` | number
`baseMktPrice` | number
`baseMktValue` | number
`baseRealizedPnl` | number
`baseUnrealizedPnl` | number
`chineseName` | string
`conExchMap` | Array&lt;any&gt;
`conid` | number
`contractDesc` | string
`countryCode` | string
`currency` | string
`displayRule` | [IndividualPositionDisplayRule](IndividualPositionDisplayRule.md)
`exchs` | object
`exerciseStyle` | string
`expiry` | string
`fullName` | string
`group` | string
`hasOptions` | boolean
`incrementRules` | [Array&lt;IndividualPositionIncrementRulesInner&gt;](IndividualPositionIncrementRulesInner.md)
`isEventContract` | boolean
`isUS` | boolean
`lastTradingDay` | string
`listingExchange` | string
`mktPrice` | number
`mktValue` | number
`model` | string
`multiplier` | number
`name` | string
`pageSize` | number
`position` | number
`putOrCall` | string
`realizedPnl` | number
`sector` | string
`sectorGroup` | string
`strike` | string
`ticker` | string
`time` | number
`type` | string
`undConid` | number
`unrealizedPnl` | number

## Example

```typescript
import type { IndividualPosition } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "acctId": null,
  "allExchanges": null,
  "assetClass": null,
  "avgCost": null,
  "avgPrice": null,
  "baseAvgCost": null,
  "baseAvgPrice": null,
  "baseMktPrice": null,
  "baseMktValue": null,
  "baseRealizedPnl": null,
  "baseUnrealizedPnl": null,
  "chineseName": null,
  "conExchMap": null,
  "conid": null,
  "contractDesc": null,
  "countryCode": null,
  "currency": null,
  "displayRule": null,
  "exchs": null,
  "exerciseStyle": null,
  "expiry": null,
  "fullName": null,
  "group": null,
  "hasOptions": null,
  "incrementRules": null,
  "isEventContract": null,
  "isUS": null,
  "lastTradingDay": null,
  "listingExchange": null,
  "mktPrice": null,
  "mktValue": null,
  "model": null,
  "multiplier": null,
  "name": null,
  "pageSize": null,
  "position": null,
  "putOrCall": null,
  "realizedPnl": null,
  "sector": null,
  "sectorGroup": null,
  "strike": null,
  "ticker": null,
  "time": null,
  "type": null,
  "undConid": null,
  "unrealizedPnl": null,
} satisfies IndividualPosition

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as IndividualPosition
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


