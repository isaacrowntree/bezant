
# AlertDetails

details of the specified alert

## Properties

Name | Type
------------ | -------------
`account` | string
`alertName` | string
`alert_active` | number
`alert_default_type` | number
`alert_email` | string
`alert_message` | string
`alert_mta_currency` | string
`alert_mta_defaults` | string
`alert_play_audio` | number
`alert_repeatable` | number
`alert_send_message` | number
`alert_show_popup` | number
`alert_triggered` | number
`bg_color` | string
`condition_outside_rth` | number
`condition_size` | number
`conditions` | [Array&lt;AlertCondition&gt;](AlertCondition.md)
`expire_time` | string
`fg_color` | string
`itws_orders_only` | number
`order_id` | number
`order_not_editable` | boolean
`order_status` | string
`tif` | string
`time_zone` | string
`tool_id` | number

## Example

```typescript
import type { AlertDetails } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "account": null,
  "alertName": null,
  "alert_active": null,
  "alert_default_type": null,
  "alert_email": null,
  "alert_message": null,
  "alert_mta_currency": null,
  "alert_mta_defaults": null,
  "alert_play_audio": null,
  "alert_repeatable": null,
  "alert_send_message": null,
  "alert_show_popup": null,
  "alert_triggered": null,
  "bg_color": null,
  "condition_outside_rth": null,
  "condition_size": null,
  "conditions": null,
  "expire_time": null,
  "fg_color": null,
  "itws_orders_only": null,
  "order_id": null,
  "order_not_editable": null,
  "order_status": null,
  "tif": null,
  "time_zone": null,
  "tool_id": null,
} satisfies AlertDetails

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AlertDetails
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


