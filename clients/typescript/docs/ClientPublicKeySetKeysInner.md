
# ClientPublicKeySetKeysInner


## Properties

Name | Type
------------ | -------------
`alg` | string
`kid` | string
`kty` | string
`use` | string
`e` | string
`n` | string
`crv` | string
`x` | string
`y` | string
`k` | string

## Example

```typescript
import type { ClientPublicKeySetKeysInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "alg": null,
  "kid": null,
  "kty": null,
  "use": null,
  "e": null,
  "n": null,
  "crv": null,
  "x": null,
  "y": null,
  "k": null,
} satisfies ClientPublicKeySetKeysInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ClientPublicKeySetKeysInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


