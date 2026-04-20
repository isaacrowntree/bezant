
# ServerPublicKey


## Properties

Name | Type
------------ | -------------
`asymmetric` | boolean
`empty` | boolean
`keyBitSize` | number
`keyId` | string
`_private` | boolean
`_public` | boolean
`symmetric` | boolean

## Example

```typescript
import type { ServerPublicKey } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "asymmetric": null,
  "empty": null,
  "keyBitSize": null,
  "keyId": null,
  "_private": null,
  "_public": null,
  "symmetric": null,
} satisfies ServerPublicKey

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ServerPublicKey
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


