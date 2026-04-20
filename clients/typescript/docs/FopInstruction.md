
# FopInstruction


## Properties

Name | Type
------------ | -------------
`accountId` | string
`clientInstructionId` | number
`contraBrokerAccountId` | string
`contraBrokerDtcCode` | string
`direction` | string
`quantity` | number
`tradingInstrument` | [TradingInstrument](TradingInstrument.md)

## Example

```typescript
import type { FopInstruction } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": U46377,
  "clientInstructionId": 1012983,
  "contraBrokerAccountId": 12345678A,
  "contraBrokerDtcCode": 534,
  "direction": IN,
  "quantity": 1000,
  "tradingInstrument": null,
} satisfies FopInstruction

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as FopInstruction
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


