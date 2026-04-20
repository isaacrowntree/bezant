
# RegistrationTask


## Properties

Name | Type
------------ | -------------
`action` | string
`dateCompleted` | Date
`externalId` | string
`formName` | string
`formNumber` | number
`isCompleted` | boolean
`isDeclined` | boolean
`isRequiredForApproval` | boolean
`questionIds` | Array&lt;number&gt;
`state` | string
`warning` | string

## Example

```typescript
import type { RegistrationTask } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "action": null,
  "dateCompleted": null,
  "externalId": null,
  "formName": null,
  "formNumber": null,
  "isCompleted": null,
  "isDeclined": null,
  "isRequiredForApproval": null,
  "questionIds": null,
  "state": null,
  "warning": null,
} satisfies RegistrationTask

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as RegistrationTask
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


