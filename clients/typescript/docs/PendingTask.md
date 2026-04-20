
# PendingTask


## Properties

Name | Type
------------ | -------------
`action` | string
`au10tixCreatedDate` | Date
`au10tixExpiryDate` | Date
`documentRejectReason` | Array&lt;string&gt;
`entityId` | number
`externalId` | string
`formName` | string
`formNumber` | number
`onlineTask` | boolean
`questionIds` | Array&lt;number&gt;
`requiredForApproval` | boolean
`requiredForTrading` | boolean
`startDate` | Date
`state` | string
`taskNumber` | number
`url` | string

## Example

```typescript
import type { PendingTask } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "action": null,
  "au10tixCreatedDate": null,
  "au10tixExpiryDate": null,
  "documentRejectReason": null,
  "entityId": null,
  "externalId": null,
  "formName": null,
  "formNumber": null,
  "onlineTask": null,
  "questionIds": null,
  "requiredForApproval": null,
  "requiredForTrading": null,
  "startDate": null,
  "state": null,
  "taskNumber": null,
  "url": null,
} satisfies PendingTask

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PendingTask
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


