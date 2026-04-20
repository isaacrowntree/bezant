
# SingleWatchlistEntry

Object containing watchlist entry for a single instrument.

## Properties

Name | Type
------------ | -------------
`C` | string
`ST` | string
`assetClass` | string
`chineseName` | string
`conid` | number
`fullName` | string
`name` | string
`ticker` | string

## Example

```typescript
import type { SingleWatchlistEntry } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "C": null,
  "ST": null,
  "assetClass": null,
  "chineseName": null,
  "conid": null,
  "fullName": null,
  "name": null,
  "ticker": null,
} satisfies SingleWatchlistEntry

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SingleWatchlistEntry
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


