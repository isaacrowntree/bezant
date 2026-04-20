
# WatchlistsResponseData

Contains the watchlist query results.

## Properties

Name | Type
------------ | -------------
`bulk_delete` | boolean
`scanners_only` | boolean
`show_scanners` | boolean
`user_lists` | [Array&lt;WatchlistsResponseDataUserListsInner&gt;](WatchlistsResponseDataUserListsInner.md)

## Example

```typescript
import type { WatchlistsResponseData } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "bulk_delete": null,
  "scanners_only": null,
  "show_scanners": null,
  "user_lists": null,
} satisfies WatchlistsResponseData

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as WatchlistsResponseData
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


