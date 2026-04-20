
# ForecastMarketResponseContractsInner


## Properties

Name | Type
------------ | -------------
`conid` | number
`expiration` | string
`expiry_label` | string
`side` | string
`strike` | number
`strike_label` | string
`time_specifier` | string
`underlying_conid` | number

## Example

```typescript
import type { ForecastMarketResponseContractsInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "conid": null,
  "expiration": null,
  "expiry_label": null,
  "side": null,
  "strike": null,
  "strike_label": null,
  "time_specifier": null,
  "underlying_conid": null,
} satisfies ForecastMarketResponseContractsInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ForecastMarketResponseContractsInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


