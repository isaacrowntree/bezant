
# ModelPositionResponse


## Properties

Name | Type
------------ | -------------
`cash` | [Array&lt;ModelPositionResponseCashInner&gt;](ModelPositionResponseCashInner.md)
`mismatched` | boolean
`model` | string
`nlv` | number
`positionList` | [Array&lt;ModelPositionResponsePositionListInner&gt;](ModelPositionResponsePositionListInner.md)
`positionTs` | number
`reqID` | number
`stkOnly` | boolean
`subscriptionStatus` | number
`totalDlv` | number
`totalMv` | number

## Example

```typescript
import type { ModelPositionResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "cash": null,
  "mismatched": null,
  "model": null,
  "nlv": null,
  "positionList": null,
  "positionTs": null,
  "reqID": null,
  "stkOnly": null,
  "subscriptionStatus": null,
  "totalDlv": null,
  "totalMv": null,
} satisfies ModelPositionResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ModelPositionResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


