
# ProcessDocuments


## Properties

Name | Type
------------ | -------------
`documents` | [Array&lt;Document&gt;](Document.md)
`inputLanguage` | string
`translation` | boolean

## Example

```typescript
import type { ProcessDocuments } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "documents": null,
  "inputLanguage": null,
  "translation": null,
} satisfies ProcessDocuments

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ProcessDocuments
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


