
# ExtPositionsTransferType


## Properties

Name | Type
------------ | -------------
`accountAtBroker` | string
`approximateAccountValue` | number
`authorizeToRemoveFund` | boolean
`brokerId` | string
`brokerName` | string
`ein` | string
`ibAccount` | string
`marginLoan` | boolean
`optionPos` | boolean
`partialBondPositions` | [Array&lt;PartialBondPosition&gt;](PartialBondPosition.md)
`partialCashPositions` | [Array&lt;PartialCashPosition&gt;](PartialCashPosition.md)
`partialFundPositions` | [Array&lt;PartialFundPosition&gt;](PartialFundPosition.md)
`partialOptionPositions` | [Array&lt;PartialOptionPosition&gt;](PartialOptionPosition.md)
`partialStockPositions` | [Array&lt;PartialStockPosition&gt;](PartialStockPosition.md)
`partialWarrantPositions` | [Array&lt;PartialWarrantPosition&gt;](PartialWarrantPosition.md)
`shortPos` | boolean
`signature` | string
`srcIRAType` | string
`ssn` | string
`subType` | string
`thirdPartyType` | string
`type` | string

## Example

```typescript
import type { ExtPositionsTransferType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountAtBroker": null,
  "approximateAccountValue": null,
  "authorizeToRemoveFund": null,
  "brokerId": null,
  "brokerName": null,
  "ein": null,
  "ibAccount": null,
  "marginLoan": null,
  "optionPos": null,
  "partialBondPositions": null,
  "partialCashPositions": null,
  "partialFundPositions": null,
  "partialOptionPositions": null,
  "partialStockPositions": null,
  "partialWarrantPositions": null,
  "shortPos": null,
  "signature": null,
  "srcIRAType": null,
  "ssn": null,
  "subType": null,
  "thirdPartyType": null,
  "type": null,
} satisfies ExtPositionsTransferType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ExtPositionsTransferType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


