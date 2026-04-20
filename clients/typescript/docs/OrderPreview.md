
# OrderPreview

Projected costs and changes to margin and equity values in the account, if the order ticket were executed in full.

## Properties

Name | Type
------------ | -------------
`amount` | [OrderPreviewAmount](OrderPreviewAmount.md)
`equity` | [OrderPreviewEquity](OrderPreviewEquity.md)
`error` | string
`initial` | [OrderPreviewInitial](OrderPreviewInitial.md)
`maintenance` | [OrderPreviewMaintenance](OrderPreviewMaintenance.md)
`position` | [OrderPreviewPosition](OrderPreviewPosition.md)
`warn` | string

## Example

```typescript
import type { OrderPreview } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "amount": null,
  "equity": null,
  "error": null,
  "initial": null,
  "maintenance": null,
  "position": null,
  "warn": null,
} satisfies OrderPreview

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as OrderPreview
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


