
# ModelPresetsResponse


## Properties

Name | Type
------------ | -------------
`avoidNegativeCashInIndependent` | boolean
`closeDivestIndependentPosition` | boolean
`fullyInvestExistingLongPositions` | boolean
`keepModelOpen` | boolean
`preferCrossWithIndependent` | boolean
`preferTransferFromIndependent` | boolean
`reqID` | number
`roundAllocationQuantityToExchangeBoardLot` | boolean
`subscriptionStatus` | number
`useNonBaseCcy` | boolean
`useToleranceRange` | boolean

## Example

```typescript
import type { ModelPresetsResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "avoidNegativeCashInIndependent": null,
  "closeDivestIndependentPosition": null,
  "fullyInvestExistingLongPositions": null,
  "keepModelOpen": null,
  "preferCrossWithIndependent": null,
  "preferTransferFromIndependent": null,
  "reqID": null,
  "roundAllocationQuantityToExchangeBoardLot": null,
  "subscriptionStatus": null,
  "useNonBaseCcy": null,
  "useToleranceRange": null,
} satisfies ModelPresetsResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ModelPresetsResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


