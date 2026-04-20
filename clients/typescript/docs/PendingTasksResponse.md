
# PendingTasksResponse


## Properties

Name | Type
------------ | -------------
`accountId` | string
`description` | string
`error` | [ErrorResponse](ErrorResponse.md)
`errorDescription` | string
`hasError` | boolean
`pendingTaskPresent` | boolean
`pendingTasks` | [Array&lt;PendingTask&gt;](PendingTask.md)
`state` | string
`status` | string

## Example

```typescript
import type { PendingTasksResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": null,
  "description": null,
  "error": null,
  "errorDescription": null,
  "hasError": null,
  "pendingTaskPresent": null,
  "pendingTasks": null,
  "state": null,
  "status": null,
} satisfies PendingTasksResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PendingTasksResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


