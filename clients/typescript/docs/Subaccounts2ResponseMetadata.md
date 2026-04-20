
# Subaccounts2ResponseMetadata

Metadata container describing the subaccounts.

## Properties

Name | Type
------------ | -------------
`pageNum` | number
`pageSize` | number
`total` | number

## Example

```typescript
import type { Subaccounts2ResponseMetadata } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "pageNum": null,
  "pageSize": null,
  "total": null,
} satisfies Subaccounts2ResponseMetadata

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Subaccounts2ResponseMetadata
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


