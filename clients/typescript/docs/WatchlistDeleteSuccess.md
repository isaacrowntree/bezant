
# WatchlistDeleteSuccess

Object detailing the successful deletion of a watchlist.

## Properties

Name | Type
------------ | -------------
`MID` | string
`action` | string
`data` | [WatchlistDeleteSuccessData](WatchlistDeleteSuccessData.md)

## Example

```typescript
import type { WatchlistDeleteSuccess } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "MID": null,
  "action": null,
  "data": null,
} satisfies WatchlistDeleteSuccess

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as WatchlistDeleteSuccess
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


