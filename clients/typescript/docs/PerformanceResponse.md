
# PerformanceResponse


## Properties

Name | Type
------------ | -------------
`cps` | [PerformanceResponseCps](PerformanceResponseCps.md)
`currencyType` | string
`id` | string
`included` | Array&lt;any&gt;
`nav` | [PerformanceResponseNav](PerformanceResponseNav.md)
`nd` | number
`pm` | string
`rc` | number
`tpps` | [PerformanceResponseTpps](PerformanceResponseTpps.md)

## Example

```typescript
import type { PerformanceResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "cps": null,
  "currencyType": null,
  "id": null,
  "included": null,
  "nav": null,
  "nd": null,
  "pm": null,
  "rc": null,
  "tpps": null,
} satisfies PerformanceResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PerformanceResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


