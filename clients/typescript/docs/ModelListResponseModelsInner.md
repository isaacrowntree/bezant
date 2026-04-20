
# ModelListResponseModelsInner


## Properties

Name | Type
------------ | -------------
`bootstrapped` | boolean
`customIndex` | boolean
`desc` | string
`isStatic` | boolean
`mismatch` | boolean
`model` | string
`nlv` | number
`numAccounts` | number

## Example

```typescript
import type { ModelListResponseModelsInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "bootstrapped": null,
  "customIndex": null,
  "desc": null,
  "isStatic": null,
  "mismatch": null,
  "model": null,
  "nlv": null,
  "numAccounts": null,
} satisfies ModelListResponseModelsInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ModelListResponseModelsInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


