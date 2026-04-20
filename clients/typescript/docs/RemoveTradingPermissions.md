
# RemoveTradingPermissions


## Properties

Name | Type
------------ | -------------
`accountId` | string
`tradingPermissions` | [Array&lt;TradingPermission&gt;](TradingPermission.md)

## Example

```typescript
import type { RemoveTradingPermissions } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": null,
  "tradingPermissions": null,
} satisfies RemoveTradingPermissions

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as RemoveTradingPermissions
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


