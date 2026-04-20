
# ModelListResponse


## Properties

Name | Type
------------ | -------------
`baseCcy` | string
`masterAccount` | string
`models` | [Array&lt;ModelListResponseModelsInner&gt;](ModelListResponseModelsInner.md)
`reqID` | number
`subscriptionStatus` | number

## Example

```typescript
import type { ModelListResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "baseCcy": null,
  "masterAccount": null,
  "models": null,
  "reqID": 540607,
  "subscriptionStatus": null,
} satisfies ModelListResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ModelListResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


