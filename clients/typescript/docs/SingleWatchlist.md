
# SingleWatchlist

Object detailing a single watchlist.

## Properties

Name | Type
------------ | -------------
`hash` | string
`id` | string
`instruments` | [Array&lt;SingleWatchlistEntry&gt;](SingleWatchlistEntry.md)
`name` | string
`readOnly` | boolean

## Example

```typescript
import type { SingleWatchlist } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "hash": null,
  "id": null,
  "instruments": null,
  "name": null,
  "readOnly": null,
} satisfies SingleWatchlist

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SingleWatchlist
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


