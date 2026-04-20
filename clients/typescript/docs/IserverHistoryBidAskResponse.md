
# IserverHistoryBidAskResponse

Object containing the requested historical data and related metadata.

## Properties

Name | Type
------------ | -------------
`barLength` | number
`chartPanStartTime` | string
`data` | [Array&lt;SingleHistoricalBarBidAsk&gt;](SingleHistoricalBarBidAsk.md)
`direction` | number
`high` | string
`low` | string
`mdAvailability` | string
`messageVersion` | number
`mktDataDelay` | number
`negativeCapable` | boolean
`outsideRth` | boolean
`points` | number
`priceDisplayRule` | number
`priceDisplayValue` | string
`priceFactor` | number
`serverId` | string
`startTime` | string
`symbol` | string
`text` | string
`timePeriod` | string
`tradingDayDuration` | number
`travelTime` | number
`volumeFactor` | number

## Example

```typescript
import type { IserverHistoryBidAskResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "barLength": null,
  "chartPanStartTime": null,
  "data": null,
  "direction": null,
  "high": null,
  "low": null,
  "mdAvailability": null,
  "messageVersion": null,
  "mktDataDelay": null,
  "negativeCapable": null,
  "outsideRth": null,
  "points": null,
  "priceDisplayRule": null,
  "priceDisplayValue": null,
  "priceFactor": null,
  "serverId": null,
  "startTime": null,
  "symbol": null,
  "text": null,
  "timePeriod": null,
  "tradingDayDuration": null,
  "travelTime": null,
  "volumeFactor": null,
} satisfies IserverHistoryBidAskResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as IserverHistoryBidAskResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


