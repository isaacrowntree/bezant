
# InterestMarkupType


## Properties

Name | Type
------------ | -------------
`creditMarkdown` | number
`currency` | string
`debitMarkup` | number
`ibDebitMarkup` | number
`longCfdDebitMarkdown` | number
`longFxCfdMarkdown` | number
`longIndexCfdDebitMarkdown` | number
`shortCfdCreditMarkdown` | number
`shortCreditMarkdown` | number
`shortFxCfdMarkup` | number
`shortIndexCfdCreditMarkdown` | number

## Example

```typescript
import type { InterestMarkupType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "creditMarkdown": null,
  "currency": null,
  "debitMarkup": null,
  "ibDebitMarkup": null,
  "longCfdDebitMarkdown": null,
  "longFxCfdMarkdown": null,
  "longIndexCfdDebitMarkdown": null,
  "shortCfdCreditMarkdown": null,
  "shortCreditMarkdown": null,
  "shortFxCfdMarkup": null,
  "shortIndexCfdCreditMarkdown": null,
} satisfies InterestMarkupType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as InterestMarkupType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


