
# RegistrationTasksResponse


## Properties

Name | Type
------------ | -------------
`accountId` | string
`description` | string
`error` | [ErrorResponse](ErrorResponse.md)
`errorDescription` | string
`hasError` | boolean
`registrationTaskPresent` | boolean
`registrationTasks` | [Array&lt;RegistrationTask&gt;](RegistrationTask.md)
`state` | string
`status` | string

## Example

```typescript
import type { RegistrationTasksResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": null,
  "description": null,
  "error": null,
  "errorDescription": null,
  "hasError": null,
  "registrationTaskPresent": null,
  "registrationTasks": null,
  "state": null,
  "status": null,
} satisfies RegistrationTasksResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as RegistrationTasksResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


