
# PerformanceResponseNav

Net asset value data for the account or consolidated accounts. NAV data is not applicable to benchmarks.

## Properties

Name | Type
------------ | -------------
`data` | [Array&lt;PerformanceResponseNavDataInner&gt;](PerformanceResponseNavDataInner.md)
`dates` | Array&lt;any&gt;
`freq` | string

## Example

```typescript
import type { PerformanceResponseNav } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "data": null,
  "dates": null,
  "freq": null,
} satisfies PerformanceResponseNav

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PerformanceResponseNav
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


