
# OrderStatus

Object containing information about the status of an order ticket.

## Properties

Name | Type
------------ | -------------
`account` | string
`alert_active` | number
`allowed_duplicate_opposite` | boolean
`available_chart_periods` | string
`average_price` | string
`bgColor` | string
`cannot_cancel_order` | boolean
`child_order_type` | string
`company_name` | string
`conid` | number
`conidex` | string
`contract_description_1` | string
`cum_fill` | string
`currency` | string
`deactivate_order` | boolean
`editable_fields` | string
`exit_strategy_chart_description` | string
`exit_strategy_display_price` | string
`exit_strategy_tool_availability` | string
`fgColor` | string
`listing_exchange` | string
`option_acct` | string
`order_ccp_status` | string
`order_clearing_account` | string
`order_description` | string
`order_description_with_contract` | string
`order_id` | number
`order_not_editable` | boolean
`order_status` | string
`order_status_description` | string
`order_time` | string
`order_type` | string
`request_id` | string
`sec_type` | string
`server_id` | string
`side` | string
`size` | string
`size_and_fills` | string
`sub_type` | string
`symbol` | string
`tif` | string
`total_size` | string

## Example

```typescript
import type { OrderStatus } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "account": null,
  "alert_active": null,
  "allowed_duplicate_opposite": null,
  "available_chart_periods": null,
  "average_price": null,
  "bgColor": null,
  "cannot_cancel_order": null,
  "child_order_type": null,
  "company_name": null,
  "conid": null,
  "conidex": null,
  "contract_description_1": null,
  "cum_fill": null,
  "currency": null,
  "deactivate_order": null,
  "editable_fields": null,
  "exit_strategy_chart_description": null,
  "exit_strategy_display_price": null,
  "exit_strategy_tool_availability": null,
  "fgColor": null,
  "listing_exchange": null,
  "option_acct": null,
  "order_ccp_status": null,
  "order_clearing_account": null,
  "order_description": null,
  "order_description_with_contract": null,
  "order_id": null,
  "order_not_editable": null,
  "order_status": null,
  "order_status_description": null,
  "order_time": null,
  "order_type": null,
  "request_id": null,
  "sec_type": null,
  "server_id": null,
  "side": null,
  "size": null,
  "size_and_fills": null,
  "sub_type": null,
  "symbol": null,
  "tif": null,
  "total_size": null,
} satisfies OrderStatus

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as OrderStatus
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


