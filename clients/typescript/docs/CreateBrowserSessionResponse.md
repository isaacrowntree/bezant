
# CreateBrowserSessionResponse


## Properties

Name | Type
------------ | -------------
`active` | boolean
`url` | string

## Example

```typescript
import type { CreateBrowserSessionResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "active": null,
  "url": https://www.interactivebrokers.com/sso/...,
} satisfies CreateBrowserSessionResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as CreateBrowserSessionResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


