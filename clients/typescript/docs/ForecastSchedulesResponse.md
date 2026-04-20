
# ForecastSchedulesResponse


## Properties

Name | Type
------------ | -------------
`timezone` | string
`trading_schedules` | [Array&lt;ForecastSchedulesResponseTradingSchedulesInner&gt;](ForecastSchedulesResponseTradingSchedulesInner.md)

## Example

```typescript
import type { ForecastSchedulesResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "timezone": null,
  "trading_schedules": null,
} satisfies ForecastSchedulesResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ForecastSchedulesResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


