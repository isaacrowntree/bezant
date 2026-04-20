
# Alert

An array containing all alerts as separate objects.

## Properties

Name | Type
------------ | -------------
`account` | string
`alert_active` | number
`alert_name` | string
`alert_repeatable` | number
`alert_triggered` | boolean
`order_id` | number
`order_time` | string

## Example

```typescript
import type { Alert } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "account": null,
  "alert_active": null,
  "alert_name": null,
  "alert_repeatable": null,
  "alert_triggered": null,
  "order_id": null,
  "order_time": null,
} satisfies Alert

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Alert
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


