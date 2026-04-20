
# AuthorizationCode


## Properties

Name | Type
------------ | -------------
`code` | string
`hash` | string
`hash_algorithm` | string

## Example

```typescript
import type { AuthorizationCode } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "code": null,
  "hash": null,
  "hash_algorithm": null,
} satisfies AuthorizationCode

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AuthorizationCode
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


