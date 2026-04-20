
# ORGRegulatorType


## Properties

Name | Type
------------ | -------------
`regulatedInCapacity` | string
`regulatorCountry` | string
`regulatorId` | string
`regulatorName` | string

## Example

```typescript
import type { ORGRegulatorType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "regulatedInCapacity": null,
  "regulatorCountry": null,
  "regulatorId": null,
  "regulatorName": null,
} satisfies ORGRegulatorType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ORGRegulatorType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


