
# TradingInstrument


## Properties

Name | Type
------------ | -------------
`currency` | string
`conid` | number
`tradingInstrumentDescription` | [TradingInstrumentOneOf1TradingInstrumentDescription](TradingInstrumentOneOf1TradingInstrumentDescription.md)

## Example

```typescript
import type { TradingInstrument } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "currency": USD,
  "conid": 459200101,
  "tradingInstrumentDescription": null,
} satisfies TradingInstrument

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TradingInstrument
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


