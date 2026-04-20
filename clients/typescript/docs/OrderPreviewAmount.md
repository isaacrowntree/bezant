
# OrderPreviewAmount

Describes the projected costs associated with the order ticket.

## Properties

Name | Type
------------ | -------------
`amount` | string
`commission` | string
`total` | string

## Example

```typescript
import type { OrderPreviewAmount } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "amount": null,
  "commission": null,
  "total": null,
} satisfies OrderPreviewAmount

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as OrderPreviewAmount
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


