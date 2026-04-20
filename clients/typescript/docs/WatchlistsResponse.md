
# WatchlistsResponse

Object containing a successful query for watchlists saved for the username in use in the current Web API session.

## Properties

Name | Type
------------ | -------------
`MID` | string
`action` | string
`data` | [WatchlistsResponseData](WatchlistsResponseData.md)

## Example

```typescript
import type { WatchlistsResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "MID": null,
  "action": null,
  "data": null,
} satisfies WatchlistsResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as WatchlistsResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


