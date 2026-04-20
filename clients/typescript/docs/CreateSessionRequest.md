
# CreateSessionRequest


## Properties

Name | Type
------------ | -------------
`alternativeIps` | Array&lt;string&gt;
`credential` | string
`ip` | string
`service` | string

## Example

```typescript
import type { CreateSessionRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "alternativeIps": ["10.10.10.11","10.10.10.12"],
  "credential": ddowney2,
  "ip": 10.10.10.10,
  "service": null,
} satisfies CreateSessionRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as CreateSessionRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


