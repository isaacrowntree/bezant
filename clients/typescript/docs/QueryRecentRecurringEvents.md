
# QueryRecentRecurringEvents


## Properties

Name | Type
------------ | -------------
`clientInstructionId` | number
`ibReferenceId` | number
`numberOfTransactions` | number

## Example

```typescript
import type { QueryRecentRecurringEvents } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "clientInstructionId": 1012983,
  "ibReferenceId": -343872793,
  "numberOfTransactions": 15,
} satisfies QueryRecentRecurringEvents

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as QueryRecentRecurringEvents
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


