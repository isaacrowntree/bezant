
# ModelPositionResponsePositionListInner


## Properties

Name | Type
------------ | -------------
`actual` | number
`actualRangeMax` | number
`actualRangeMin` | number
`ccy` | string
`conid` | number
`dlv` | number
`exchangeRate` | number
`flags` | number
`instrument` | string
`instrumentImbalance` | number
`mismatchType` | number
`mv` | number
`position` | number
`target` | number

## Example

```typescript
import type { ModelPositionResponsePositionListInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "actual": null,
  "actualRangeMax": null,
  "actualRangeMin": null,
  "ccy": null,
  "conid": null,
  "dlv": null,
  "exchangeRate": null,
  "flags": null,
  "instrument": null,
  "instrumentImbalance": null,
  "mismatchType": null,
  "mv": null,
  "position": null,
  "target": null,
} satisfies ModelPositionResponsePositionListInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ModelPositionResponsePositionListInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


