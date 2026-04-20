
# LocalTaxForm


## Properties

Name | Type
------------ | -------------
`qualified` | boolean
`taxAuthority` | string
`treatyCountry` | string

## Example

```typescript
import type { LocalTaxForm } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "qualified": null,
  "taxAuthority": null,
  "treatyCountry": null,
} satisfies LocalTaxForm

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as LocalTaxForm
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


