
# OrderSubmitSuccessInner

Result of successful submission of one order ticket.

## Properties

Name | Type
------------ | -------------
`encrypt_message` | string
`order_id` | string
`order_status` | string

## Example

```typescript
import type { OrderSubmitSuccessInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "encrypt_message": null,
  "order_id": null,
  "order_status": null,
} satisfies OrderSubmitSuccessInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as OrderSubmitSuccessInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


