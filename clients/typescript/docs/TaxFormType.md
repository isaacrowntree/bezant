
# TaxFormType


## Properties

Name | Type
------------ | -------------
`formats` | Array&lt;string&gt;
`isForm` | boolean
`taxFormName` | string

## Example

```typescript
import type { TaxFormType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "formats": null,
  "isForm": null,
  "taxFormName": 1099,
} satisfies TaxFormType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TaxFormType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


