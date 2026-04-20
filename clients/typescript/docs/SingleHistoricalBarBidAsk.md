
# SingleHistoricalBarBidAsk

Object containing Bid and Ask data for a single OHLC bar.

## Properties

Name | Type
------------ | -------------
`c` | number
`h` | number
`l` | number
`o` | number
`t` | number
`v` | number

## Example

```typescript
import type { SingleHistoricalBarBidAsk } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "c": null,
  "h": null,
  "l": null,
  "o": null,
  "t": null,
  "v": null,
} satisfies SingleHistoricalBarBidAsk

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SingleHistoricalBarBidAsk
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


