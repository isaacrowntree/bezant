
# AnswerResponse


## Properties

Name | Type
------------ | -------------
`answer` | string
`answerId` | number
`dependentAnswerId` | number
`dependentQuestionId` | number
`multiAnswerDetail` | Array&lt;string&gt;

## Example

```typescript
import type { AnswerResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "answer": null,
  "answerId": null,
  "dependentAnswerId": null,
  "dependentQuestionId": null,
  "multiAnswerDetail": null,
} satisfies AnswerResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AnswerResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


