
# ForecastMarketResponse


## Properties

Name | Type
------------ | -------------
`contracts` | [Array&lt;ForecastMarketResponseContractsInner&gt;](ForecastMarketResponseContractsInner.md)
`exchange` | string
`exclude_historical_data` | boolean
`logo_category` | string
`market_name` | string
`payout` | number
`symbol` | string

## Example

```typescript
import type { ForecastMarketResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "contracts": null,
  "exchange": null,
  "exclude_historical_data": null,
  "logo_category": null,
  "market_name": null,
  "payout": null,
  "symbol": null,
} satisfies ForecastMarketResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ForecastMarketResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


