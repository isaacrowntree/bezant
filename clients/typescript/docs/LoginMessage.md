
# LoginMessage


## Properties

Name | Type
------------ | -------------
`contentId` | number
`description` | string
`id` | number
`messageType` | string
`recordDate` | Date
`state` | string
`tasks` | Array&lt;number&gt;
`username` | string

## Example

```typescript
import type { LoginMessage } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "contentId": null,
  "description": null,
  "id": null,
  "messageType": null,
  "recordDate": null,
  "state": null,
  "tasks": null,
  "username": null,
} satisfies LoginMessage

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as LoginMessage
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


