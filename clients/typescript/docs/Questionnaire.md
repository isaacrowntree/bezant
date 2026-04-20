
# Questionnaire


## Properties

Name | Type
------------ | -------------
`answers` | [Array&lt;Answer&gt;](Answer.md)
`formNumber` | number

## Example

```typescript
import type { Questionnaire } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "answers": null,
  "formNumber": null,
} satisfies Questionnaire

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Questionnaire
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


