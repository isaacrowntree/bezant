
# ForecastSchedulesResponseTradingSchedulesInner


## Properties

Name | Type
------------ | -------------
`day_of_week` | string
`trading_times` | [Array&lt;ForecastSchedulesResponseTradingSchedulesInnerTradingTimesInner&gt;](ForecastSchedulesResponseTradingSchedulesInnerTradingTimesInner.md)

## Example

```typescript
import type { ForecastSchedulesResponseTradingSchedulesInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "day_of_week": null,
  "trading_times": null,
} satisfies ForecastSchedulesResponseTradingSchedulesInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ForecastSchedulesResponseTradingSchedulesInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


