
# SetAccountinvestmentInModelRequest


## Properties

Name | Type
------------ | -------------
`accountList` | [Array&lt;SetAccountinvestmentInModelRequestAccountListInner&gt;](SetAccountinvestmentInModelRequestAccountListInner.md)
`model` | string
`reqID` | number

## Example

```typescript
import type { SetAccountinvestmentInModelRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountList": null,
  "model": null,
  "reqID": null,
} satisfies SetAccountinvestmentInModelRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SetAccountinvestmentInModelRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


