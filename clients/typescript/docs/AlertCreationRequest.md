
# AlertCreationRequest


## Properties

Name | Type
------------ | -------------
`alertMessage` | string
`alertName` | string
`alertRepeatable` | number
`conditions` | [Array&lt;AlertCreationRequestConditionsInner&gt;](AlertCreationRequestConditionsInner.md)
`email` | string
`expireTime` | string
`iTWSOrdersOnly` | number
`orderId` | number
`outsideRth` | number
`sendMessage` | number
`showPopup` | number
`tif` | string

## Example

```typescript
import type { AlertCreationRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "alertMessage": null,
  "alertName": null,
  "alertRepeatable": null,
  "conditions": null,
  "email": null,
  "expireTime": null,
  "iTWSOrdersOnly": null,
  "orderId": null,
  "outsideRth": null,
  "sendMessage": null,
  "showPopup": null,
  "tif": null,
} satisfies AlertCreationRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AlertCreationRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


