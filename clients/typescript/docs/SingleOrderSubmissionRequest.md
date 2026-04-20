
# SingleOrderSubmissionRequest

A single order ticket.

## Properties

Name | Type
------------ | -------------
`acctId` | string
`allOrNone` | boolean
`auxPrice` | number
`cOID` | string
`cashQty` | number
`conid` | number
`conidex` | string
`extOperator` | string
`isCcyConv` | boolean
`isSingleGroup` | boolean
`jsonPayload` | [SingleOrderSubmissionRequestJsonPayload](SingleOrderSubmissionRequestJsonPayload.md)
`listingExchange` | string
`manualIndicator` | boolean
`orderType` | string
`outsideRTH` | boolean
`parentId` | string
`price` | number
`quantity` | number
`referrer` | string
`secType` | string
`side` | string
`strategy` | string
`strategyParameters` | [SingleOrderSubmissionRequestStrategyParameters](SingleOrderSubmissionRequestStrategyParameters.md)
`taxOptimizerId` | string
`ticker` | string
`tif` | string
`trailingAmt` | number
`trailingType` | string
`useAdaptive` | boolean

## Example

```typescript
import type { SingleOrderSubmissionRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "acctId": null,
  "allOrNone": null,
  "auxPrice": null,
  "cOID": null,
  "cashQty": null,
  "conid": null,
  "conidex": null,
  "extOperator": null,
  "isCcyConv": null,
  "isSingleGroup": null,
  "jsonPayload": null,
  "listingExchange": null,
  "manualIndicator": null,
  "orderType": null,
  "outsideRTH": null,
  "parentId": null,
  "price": null,
  "quantity": null,
  "referrer": null,
  "secType": null,
  "side": null,
  "strategy": null,
  "strategyParameters": null,
  "taxOptimizerId": null,
  "ticker": null,
  "tif": null,
  "trailingAmt": null,
  "trailingType": null,
  "useAdaptive": null,
} satisfies SingleOrderSubmissionRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SingleOrderSubmissionRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


