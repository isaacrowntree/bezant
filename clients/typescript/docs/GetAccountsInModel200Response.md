
# GetAccountsInModel200Response


## Properties

Name | Type
------------ | -------------
`accountInfoList` | [Array&lt;GetAccountsInModel200ResponseAccountInfoListInner&gt;](GetAccountsInModel200ResponseAccountInfoListInner.md)
`baseCcyMaster` | string
`model` | string
`reqID` | number

## Example

```typescript
import type { GetAccountsInModel200Response } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountInfoList": null,
  "baseCcyMaster": null,
  "model": null,
  "reqID": null,
} satisfies GetAccountsInModel200Response

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GetAccountsInModel200Response
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


