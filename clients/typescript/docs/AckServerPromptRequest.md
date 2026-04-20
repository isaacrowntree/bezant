
# AckServerPromptRequest


## Properties

Name | Type
------------ | -------------
`orderId` | number
`reqId` | string
`text` | string

## Example

```typescript
import type { AckServerPromptRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "orderId": 987654321,
  "reqId": 12345,
  "text": Yes,
} satisfies AckServerPromptRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AckServerPromptRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


