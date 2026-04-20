
# ComplexAssetTransferInstruction


## Properties

Name | Type
------------ | -------------
`accountId` | string
`accountIdAtCurrentBroker` | string
`clientInstructionId` | number
`contraBrokerInfo` | [ContraBrokerInfo](ContraBrokerInfo.md)
`direction` | string
`nonDisclosedDetail` | [NonDisclosedDetail](NonDisclosedDetail.md)
`quantity` | number
`tradingInstrument` | [TradingInstrument](TradingInstrument.md)

## Example

```typescript
import type { ComplexAssetTransferInstruction } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": U46377,
  "accountIdAtCurrentBroker": 7NXXXX0,
  "clientInstructionId": 1012983,
  "contraBrokerInfo": null,
  "direction": IN,
  "nonDisclosedDetail": null,
  "quantity": 1000,
  "tradingInstrument": null,
} satisfies ComplexAssetTransferInstruction

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ComplexAssetTransferInstruction
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


