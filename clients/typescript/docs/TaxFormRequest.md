
# TaxFormRequest


## Properties

Name | Type
------------ | -------------
`accountId` | string
`format` | string
`gzip` | boolean
`type` | string
`year` | number

## Example

```typescript
import type { TaxFormRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": UXXXX,
  "format": HTML,CSV,PDF,
  "gzip": null,
  "type": ALL,1099,1099R,1042S,8949,
  "year": 2023,
} satisfies TaxFormRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TaxFormRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


