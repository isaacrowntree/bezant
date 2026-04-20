
# QuizQuestionnaires


## Properties

Name | Type
------------ | -------------
`accountId` | string
`questionnaire` | [Array&lt;Questionnaire&gt;](Questionnaire.md)
`task` | [Array&lt;Task&gt;](Task.md)

## Example

```typescript
import type { QuizQuestionnaires } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": null,
  "questionnaire": null,
  "task": null,
} satisfies QuizQuestionnaires

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as QuizQuestionnaires
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


