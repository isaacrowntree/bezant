
# ForecastRulesResponse


## Properties

Name | Type
------------ | -------------
`asset_class` | string
`data_and_resolution_link` | string
`description` | string
`exchange_timezone` | string
`last_trade_time` | number
`market_name` | string
`market_rules_link` | string
`measured_period` | string
`payout` | string
`payout_time` | number
`price_increment` | string
`product_code` | string
`release_time` | number
`source_agency` | string
`threshold` | string

## Example

```typescript
import type { ForecastRulesResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "asset_class": null,
  "data_and_resolution_link": null,
  "description": null,
  "exchange_timezone": null,
  "last_trade_time": null,
  "market_name": null,
  "market_rules_link": null,
  "measured_period": null,
  "payout": null,
  "payout_time": null,
  "price_increment": null,
  "product_code": null,
  "release_time": null,
  "source_agency": null,
  "threshold": null,
} satisfies ForecastRulesResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ForecastRulesResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


