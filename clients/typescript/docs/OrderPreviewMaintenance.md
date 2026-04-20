
# OrderPreviewMaintenance

Describes the projected change to maintenance margin.

## Properties

Name | Type
------------ | -------------
`after` | string
`change` | string
`current` | string

## Example

```typescript
import type { OrderPreviewMaintenance } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "after": null,
  "change": null,
  "current": null,
} satisfies OrderPreviewMaintenance

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as OrderPreviewMaintenance
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


