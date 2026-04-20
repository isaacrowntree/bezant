
# TradesResponseInner

Object containing details of a single execution.

## Properties

Name | Type
------------ | -------------
`account` | string
`accountCode` | string
`account_allocation_name` | string
`clearing_id` | string
`clearing_name` | string
`commission` | string
`company_name` | string
`conid` | string
`conidEx` | string
`contract_description_1` | string
`exchange` | string
`execution_id` | string
`is_event_trading` | string
`liquidation_trade` | string
`listing_exchange` | string
`net_amount` | number
`order_description` | string
`order_id` | number
`order_ref` | string
`price` | string
`sec_type` | string
`side` | string
`size` | number
`submitter` | string
`supports_tax_opt` | string
`symbol` | string
`trade_time` | string
`trade_time_r` | number

## Example

```typescript
import type { TradesResponseInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "account": null,
  "accountCode": null,
  "account_allocation_name": null,
  "clearing_id": null,
  "clearing_name": null,
  "commission": null,
  "company_name": null,
  "conid": null,
  "conidEx": null,
  "contract_description_1": null,
  "exchange": null,
  "execution_id": null,
  "is_event_trading": null,
  "liquidation_trade": null,
  "listing_exchange": null,
  "net_amount": null,
  "order_description": null,
  "order_id": null,
  "order_ref": null,
  "price": null,
  "sec_type": null,
  "side": null,
  "size": null,
  "submitter": null,
  "supports_tax_opt": null,
  "symbol": null,
  "trade_time": null,
  "trade_time_r": null,
} satisfies TradesResponseInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TradesResponseInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


