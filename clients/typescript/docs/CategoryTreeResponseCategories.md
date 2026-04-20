
# CategoryTreeResponseCategories

A JSON object containing all category IDs and their relevant information.

## Properties

Name | Type
------------ | -------------
`categoryId` | [CategoryTreeResponseCategoriesCategoryId](CategoryTreeResponseCategoriesCategoryId.md)

## Example

```typescript
import type { CategoryTreeResponseCategories } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "categoryId": null,
} satisfies CategoryTreeResponseCategories

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as CategoryTreeResponseCategories
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


