
# ReqLiveSessionToken200Response


## Properties

Name | Type
------------ | -------------
`diffie_hellman_challenge` | string
`live_session_token_expiration` | number
`live_session_token_signature` | string

## Example

```typescript
import type { ReqLiveSessionToken200Response } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "diffie_hellman_challenge": null,
  "live_session_token_expiration": null,
  "live_session_token_signature": null,
} satisfies ReqLiveSessionToken200Response

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ReqLiveSessionToken200Response
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


