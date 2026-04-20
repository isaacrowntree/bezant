
# FormDetails


## Properties

Name | Type
------------ | -------------
`acceptableDocs` | Array&lt;string&gt;
`action` | string
`apiSupportedTask` | boolean
`dateModified` | Date
`error` | [ErrorResponse](ErrorResponse.md)
`errorDescription` | string
`fileLength` | number
`fileName` | string
`formName` | string
`formNumber` | number
`hasError` | boolean
`language` | string
`payload` | [FormPayload](FormPayload.md)
`questionnaire` | [Array&lt;QuestionnaireResponse&gt;](QuestionnaireResponse.md)
`sha1Checksum` | string
`type` | string

## Example

```typescript
import type { FormDetails } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "acceptableDocs": null,
  "action": null,
  "apiSupportedTask": null,
  "dateModified": null,
  "error": null,
  "errorDescription": null,
  "fileLength": null,
  "fileName": null,
  "formName": null,
  "formNumber": null,
  "hasError": null,
  "language": null,
  "payload": null,
  "questionnaire": null,
  "sha1Checksum": null,
  "type": null,
} satisfies FormDetails

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as FormDetails
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


