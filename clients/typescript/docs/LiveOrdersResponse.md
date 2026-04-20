
# LiveOrdersResponse


## Properties

Name | Type
------------ | -------------
`orders` | [Array&lt;LiveOrdersResponseOrdersInner&gt;](LiveOrdersResponseOrdersInner.md)
`snapshot` | boolean

## Example

```typescript
import type { LiveOrdersResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "orders": null,
  "snapshot": null,
} satisfies LiveOrdersResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as LiveOrdersResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


