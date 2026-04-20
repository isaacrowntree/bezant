
# PerformanceResponseNavDataInner


## Properties

Name | Type
------------ | -------------
`baseCurrency` | string
`end` | string
`id` | string
`idType` | string
`navs` | Array&lt;any&gt;
`start` | string
`startNAV` | [PerformanceResponseNavDataInnerStartNAV](PerformanceResponseNavDataInnerStartNAV.md)

## Example

```typescript
import type { PerformanceResponseNavDataInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "baseCurrency": null,
  "end": null,
  "id": null,
  "idType": null,
  "navs": null,
  "start": null,
  "startNAV": null,
} satisfies PerformanceResponseNavDataInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PerformanceResponseNavDataInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


