
# ExternalPositionTransfer


## Properties

Name | Type
------------ | -------------
`accountAtBroker` | string
`accountId` | string
`brokerId` | string
`brokerName` | string
`clientInstructionId` | number
`signature` | string
`sourceIRAType` | string
`subType` | string
`type` | string

## Example

```typescript
import type { ExternalPositionTransfer } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountAtBroker": SOL12345,
  "accountId": U2323232,
  "brokerId": 0226,
  "brokerName": Wall Street Financial Group,
  "clientInstructionId": 1012983,
  "signature": sample signature,
  "sourceIRAType": RO,
  "subType": ACATS,
  "type": FULL,
} satisfies ExternalPositionTransfer

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ExternalPositionTransfer
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


