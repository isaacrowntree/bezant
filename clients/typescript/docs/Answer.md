
# Answer


## Properties

Name | Type
------------ | -------------
`answerDetail` | [Array&lt;AnswerDetail&gt;](AnswerDetail.md)
`detail` | string
`id` | number
`questionId` | number

## Example

```typescript
import type { Answer } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "answerDetail": null,
  "detail": null,
  "id": null,
  "questionId": null,
} satisfies Answer

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Answer
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


