
# QuestionnaireResponse


## Properties

Name | Type
------------ | -------------
`answers` | [Array&lt;AnswerResponse&gt;](AnswerResponse.md)
`isMandatoryToAnswer` | boolean
`question` | string
`questionId` | number
`questionType` | string

## Example

```typescript
import type { QuestionnaireResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "answers": null,
  "isMandatoryToAnswer": null,
  "question": null,
  "questionId": null,
  "questionType": null,
} satisfies QuestionnaireResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as QuestionnaireResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


