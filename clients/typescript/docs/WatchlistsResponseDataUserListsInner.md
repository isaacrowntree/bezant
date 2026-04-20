
# WatchlistsResponseDataUserListsInner

Details of a single watchlist.

## Properties

Name | Type
------------ | -------------
`id` | string
`is_open` | boolean
`modified` | number
`name` | string
`read_only` | boolean
`type` | string

## Example

```typescript
import type { WatchlistsResponseDataUserListsInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "id": null,
  "is_open": null,
  "modified": null,
  "name": null,
  "read_only": null,
  "type": null,
} satisfies WatchlistsResponseDataUserListsInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as WatchlistsResponseDataUserListsInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


