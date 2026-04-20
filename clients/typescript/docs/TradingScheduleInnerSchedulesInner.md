
# TradingScheduleInnerSchedulesInner


## Properties

Name | Type
------------ | -------------
`clearingCycleEndTime` | string
`sessions` | [Array&lt;TradingScheduleInnerSchedulesInnerSessionsInner&gt;](TradingScheduleInnerSchedulesInnerSessionsInner.md)
`tradingScheduleDate` | string
`tradingTimes` | [Array&lt;TradingScheduleInnerSchedulesInnerTradingTimesInner&gt;](TradingScheduleInnerSchedulesInnerTradingTimesInner.md)

## Example

```typescript
import type { TradingScheduleInnerSchedulesInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "clearingCycleEndTime": null,
  "sessions": null,
  "tradingScheduleDate": null,
  "tradingTimes": null,
} satisfies TradingScheduleInnerSchedulesInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TradingScheduleInnerSchedulesInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


