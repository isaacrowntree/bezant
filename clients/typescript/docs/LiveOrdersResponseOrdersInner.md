
# LiveOrdersResponseOrdersInner

Object representing one order.

## Properties

Name | Type
------------ | -------------
`account` | string
`acct` | string
`avgPrice` | string
`bgColor` | string
`cashCcy` | string
`companyName` | string
`conid` | string
`conidex` | string
`description1` | string
`description2` | string
`exchange` | string
`fgColor` | string
`filledQuantity` | string
`isEventTrading` | string
`lastExecutionTime` | string
`lastExecutionTime_r` | string
`listingExchange` | string
`orderDesc` | string
`orderId` | number
`orderType` | string
`order_cancellation_by_system_reason` | string
`order_ccp_status` | string
`origOrderType` | string
`price` | string
`remainingQuantity` | string
`secType` | string
`side` | string
`sizeAndFills` | string
`status` | string
`supportsTaxOpt` | string
`taxOptimizerId` | string
`ticker` | string
`timeInForce` | string
`totalCashSize` | string
`totalSize` | string

## Example

```typescript
import type { LiveOrdersResponseOrdersInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "account": null,
  "acct": null,
  "avgPrice": null,
  "bgColor": null,
  "cashCcy": null,
  "companyName": null,
  "conid": null,
  "conidex": null,
  "description1": null,
  "description2": null,
  "exchange": null,
  "fgColor": null,
  "filledQuantity": null,
  "isEventTrading": null,
  "lastExecutionTime": null,
  "lastExecutionTime_r": null,
  "listingExchange": null,
  "orderDesc": null,
  "orderId": null,
  "orderType": null,
  "order_cancellation_by_system_reason": null,
  "order_ccp_status": null,
  "origOrderType": null,
  "price": null,
  "remainingQuantity": null,
  "secType": null,
  "side": null,
  "sizeAndFills": null,
  "status": null,
  "supportsTaxOpt": null,
  "taxOptimizerId": null,
  "ticker": null,
  "timeInForce": null,
  "totalCashSize": null,
  "totalSize": null,
} satisfies LiveOrdersResponseOrdersInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as LiveOrdersResponseOrdersInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


