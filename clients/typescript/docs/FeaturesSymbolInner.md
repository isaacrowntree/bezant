
# FeaturesSymbolInner


## Properties

Name | Type
------------ | -------------
`conid` | number
`expirationDate` | number
`longFuturesCutOff` | number
`ltd` | number
`shortFuturesCutOff` | number
`symbol` | string
`underlyingConid` | number

## Example

```typescript
import type { FeaturesSymbolInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "conid": null,
  "expirationDate": null,
  "longFuturesCutOff": null,
  "ltd": null,
  "shortFuturesCutOff": null,
  "symbol": null,
  "underlyingConid": null,
} satisfies FeaturesSymbolInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as FeaturesSymbolInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


