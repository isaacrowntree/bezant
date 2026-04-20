
# DwacInstruction


## Properties

Name | Type
------------ | -------------
`accountId` | string
`accountTitle` | string
`clientInstructionId` | number
`contraBrokerAccountId` | string
`contraBrokerTaxId` | string
`direction` | string
`quantity` | number
`referenceId` | string
`tradingInstrument` | [TradingInstrument](TradingInstrument.md)

## Example

```typescript
import type { DwacInstruction } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": U46377,
  "accountTitle": Title,
  "clientInstructionId": 1012983,
  "contraBrokerAccountId": 12345678A,
  "contraBrokerTaxId": 123456789,
  "direction": IN,
  "quantity": 1000,
  "referenceId": refId,
  "tradingInstrument": null,
} satisfies DwacInstruction

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as DwacInstruction
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


