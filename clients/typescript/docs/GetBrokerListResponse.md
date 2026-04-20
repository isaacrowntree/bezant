
# GetBrokerListResponse


## Properties

Name | Type
------------ | -------------
`brokers` | Array&lt;string&gt;
`instructionType` | string

## Example

```typescript
import type { GetBrokerListResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "brokers": null,
  "instructionType": COMPLEX_ASSET_TRANSFER,
} satisfies GetBrokerListResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GetBrokerListResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


