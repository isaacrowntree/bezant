
# SetModelTargetPositionsRequest


## Properties

Name | Type
------------ | -------------
`cashTargets` | [Array&lt;SetModelTargetPositionsRequestCashTargetsInner&gt;](SetModelTargetPositionsRequestCashTargetsInner.md)
`desc` | string
`isStatic` | boolean
`model` | string
`positionTargets` | [Array&lt;SetModelTargetPositionsRequestPositionTargetsInner&gt;](SetModelTargetPositionsRequestPositionTargetsInner.md)
`reqID` | number

## Example

```typescript
import type { SetModelTargetPositionsRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "cashTargets": null,
  "desc": null,
  "isStatic": null,
  "model": Sample-Model,
  "positionTargets": null,
  "reqID": 540607,
} satisfies SetModelTargetPositionsRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SetModelTargetPositionsRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


