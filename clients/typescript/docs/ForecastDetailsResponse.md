
# ForecastDetailsResponse


## Properties

Name | Type
------------ | -------------
`category` | string
`conid_no` | number
`conid_yes` | number
`exchange` | string
`expiration` | string
`logo_category` | string
`market_name` | string
`measured_period` | string
`payout` | number
`question` | string
`side` | string
`strike` | number
`strike_label` | string
`symbol` | string
`underlying_conid` | number

## Example

```typescript
import type { ForecastDetailsResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "category": null,
  "conid_no": null,
  "conid_yes": null,
  "exchange": null,
  "expiration": null,
  "logo_category": null,
  "market_name": null,
  "measured_period": null,
  "payout": null,
  "question": null,
  "side": null,
  "strike": null,
  "strike_label": null,
  "symbol": null,
  "underlying_conid": null,
} satisfies ForecastDetailsResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ForecastDetailsResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


