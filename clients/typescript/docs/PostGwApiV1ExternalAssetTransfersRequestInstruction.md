
# PostGwApiV1ExternalAssetTransfersRequestInstruction


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
`accountTitle` | string
`contraBrokerTaxId` | string
`referenceId` | string
`accountIdAtCurrentBroker` | string
`contraBrokerInfo` | [ContraBrokerInfo](ContraBrokerInfo.md)
`nonDisclosedDetail` | [NonDisclosedDetail](NonDisclosedDetail.md)
`accountAtBroker` | string
`brokerId` | string
`brokerName` | string
`signature` | string
`sourceIRAType` | string
`subType` | string
`type` | string

## Example

```typescript
import type { PostGwApiV1ExternalAssetTransfersRequestInstruction } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": U2323232,
  "clientInstructionId": 1012983,
  "contraBrokerAccountId": 12345678A,
  "contraBrokerDtcCode": 534,
  "direction": IN,
  "quantity": 1000,
  "tradingInstrument": null,
  "accountTitle": Title,
  "contraBrokerTaxId": 123456789,
  "referenceId": refId,
  "accountIdAtCurrentBroker": 7NXXXX0,
  "contraBrokerInfo": null,
  "nonDisclosedDetail": null,
  "accountAtBroker": SOL12345,
  "brokerId": 0226,
  "brokerName": Wall Street Financial Group,
  "signature": sample signature,
  "sourceIRAType": RO,
  "subType": ACATS,
  "type": FULL,
} satisfies PostGwApiV1ExternalAssetTransfersRequestInstruction

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PostGwApiV1ExternalAssetTransfersRequestInstruction
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


