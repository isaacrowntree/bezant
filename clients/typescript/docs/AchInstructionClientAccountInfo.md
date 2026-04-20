
# AchInstructionClientAccountInfo


## Properties

Name | Type
------------ | -------------
`bankAccountNumber` | string
`bankAccountTypeCode` | number
`bankName` | string
`bankRoutingNumber` | string

## Example

```typescript
import type { AchInstructionClientAccountInfo } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "bankAccountNumber": 101267576983,
  "bankAccountTypeCode": 1,
  "bankName": JPM Chase,
  "bankRoutingNumber": 202012983,
} satisfies AchInstructionClientAccountInfo

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AchInstructionClientAccountInfo
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


