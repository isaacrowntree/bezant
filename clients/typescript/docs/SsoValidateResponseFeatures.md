
# SsoValidateResponseFeatures

Returns supported features such as bonds and option trading.

## Properties

Name | Type
------------ | -------------
`bond` | boolean
`calendar` | boolean
`envs` | string
`newMf` | boolean
`optionChains` | boolean
`realtime` | boolean
`wlms` | boolean

## Example

```typescript
import type { SsoValidateResponseFeatures } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "bond": null,
  "calendar": null,
  "envs": null,
  "newMf": null,
  "optionChains": null,
  "realtime": null,
  "wlms": null,
} satisfies SsoValidateResponseFeatures

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SsoValidateResponseFeatures
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


