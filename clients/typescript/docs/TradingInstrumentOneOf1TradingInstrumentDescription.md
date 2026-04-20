
# TradingInstrumentOneOf1TradingInstrumentDescription


## Properties

Name | Type
------------ | -------------
`assetType` | string
`securityId` | string
`securityIdType` | string

## Example

```typescript
import type { TradingInstrumentOneOf1TradingInstrumentDescription } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "assetType": STK,
  "securityId": 459200101,
  "securityIdType": ISIN,
} satisfies TradingInstrumentOneOf1TradingInstrumentDescription

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TradingInstrumentOneOf1TradingInstrumentDescription
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


