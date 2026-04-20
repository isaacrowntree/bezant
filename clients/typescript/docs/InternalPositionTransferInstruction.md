
# InternalPositionTransferInstruction


## Properties

Name | Type
------------ | -------------
`clientInstructionId` | number
`settleDate` | string
`sourceAccountId` | string
`targetAccountId` | string
`tradeDate` | string
`tradingInstrument` | [TradingInstrument](TradingInstrument.md)
`transferPrice` | number
`transferQuantity` | number

## Example

```typescript
import type { InternalPositionTransferInstruction } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "clientInstructionId": 1012983,
  "settleDate": 2025-02-25,
  "sourceAccountId": U46377,
  "targetAccountId": U463756,
  "tradeDate": 2025-02-17,
  "tradingInstrument": null,
  "transferPrice": 123,
  "transferQuantity": 100,
} satisfies InternalPositionTransferInstruction

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as InternalPositionTransferInstruction
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


