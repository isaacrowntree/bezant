
# GetTransactionsRequest


## Properties

Name | Type
------------ | -------------
`acctIds` | Array&lt;string&gt;
`conids` | Array&lt;number&gt;
`currency` | string
`days` | number

## Example

```typescript
import type { GetTransactionsRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "acctIds": null,
  "conids": null,
  "currency": null,
  "days": null,
} satisfies GetTransactionsRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GetTransactionsRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


