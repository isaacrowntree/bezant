
# OrderCancelSuccess

Acknowledges IB\'s acceptance of the request to cancel the order. Does not report whether the cancellation can or will ultimately be enacted.

## Properties

Name | Type
------------ | -------------
`account` | string
`conid` | string
`msg` | string
`order_id` | string

## Example

```typescript
import type { OrderCancelSuccess } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "account": null,
  "conid": null,
  "msg": null,
  "order_id": null,
} satisfies OrderCancelSuccess

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as OrderCancelSuccess
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


