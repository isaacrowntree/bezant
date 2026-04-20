
# AdvancedOrderReject

Relates a message generated in response to the rejection of the submitted order ticket. In some cases, it may also present a mechanism to resubmit the same order following a prompted decision.

## Properties

Name | Type
------------ | -------------
`dismissable` | Array&lt;any&gt;
`messageId` | string
`options` | Array&lt;string&gt;
`orderId` | number
`prompt` | boolean
`reqId` | string
`text` | string
`type` | string

## Example

```typescript
import type { AdvancedOrderReject } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "dismissable": null,
  "messageId": null,
  "options": null,
  "orderId": null,
  "prompt": null,
  "reqId": null,
  "text": null,
  "type": null,
} satisfies AdvancedOrderReject

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AdvancedOrderReject
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


