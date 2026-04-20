
# IndividualPositionDisplayRule

Object defining minimum increments used in displaying market data for the instrument.

## Properties

Name | Type
------------ | -------------
`displayRuleStep` | [Array&lt;IndividualPositionDisplayRuleDisplayRuleStepInner&gt;](IndividualPositionDisplayRuleDisplayRuleStepInner.md)
`magnification` | number

## Example

```typescript
import type { IndividualPositionDisplayRule } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "displayRuleStep": null,
  "magnification": null,
} satisfies IndividualPositionDisplayRule

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as IndividualPositionDisplayRule
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


