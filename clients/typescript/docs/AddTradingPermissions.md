
# AddTradingPermissions


## Properties

Name | Type
------------ | -------------
`accountId` | string
`documentSubmission` | [DocumentSubmission](DocumentSubmission.md)
`optionLevel` | number
`tradingPermissions` | [Array&lt;TradingPermission&gt;](TradingPermission.md)

## Example

```typescript
import type { AddTradingPermissions } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": null,
  "documentSubmission": null,
  "optionLevel": null,
  "tradingPermissions": null,
} satisfies AddTradingPermissions

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AddTradingPermissions
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


