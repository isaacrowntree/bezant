
# LoginMessageResponse


## Properties

Name | Type
------------ | -------------
`accountId` | string
`clearingStatus` | string
`clearingStatusDescription` | string
`loginMessagePresent` | boolean
`loginMessages` | [Array&lt;LoginMessage&gt;](LoginMessage.md)

## Example

```typescript
import type { LoginMessageResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": null,
  "clearingStatus": null,
  "clearingStatusDescription": null,
  "loginMessagePresent": null,
  "loginMessages": null,
} satisfies LoginMessageResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as LoginMessageResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


