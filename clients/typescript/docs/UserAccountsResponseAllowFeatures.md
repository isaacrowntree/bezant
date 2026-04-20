
# UserAccountsResponseAllowFeatures


## Properties

Name | Type
------------ | -------------
`allowCrypto` | boolean
`allowDynAccount` | boolean
`allowEventContract` | boolean
`allowEventTrading` | boolean
`allowFXConv` | boolean
`allowFinancialLens` | boolean
`allowMTA` | boolean
`allowTypeAhead` | boolean
`allowedAssetTypes` | string
`debugPnl` | boolean
`liteUser` | boolean
`research` | boolean
`restrictTradeSubscription` | boolean
`showEUCostReport` | boolean
`showGFIS` | boolean
`showImpactDashboard` | boolean
`showTaxOpt` | boolean
`showUkUserLabels` | boolean
`showWebNews` | boolean
`sideBySide` | boolean
`snapshotRefreshTimeout` | number

## Example

```typescript
import type { UserAccountsResponseAllowFeatures } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "allowCrypto": null,
  "allowDynAccount": null,
  "allowEventContract": null,
  "allowEventTrading": null,
  "allowFXConv": null,
  "allowFinancialLens": null,
  "allowMTA": null,
  "allowTypeAhead": null,
  "allowedAssetTypes": null,
  "debugPnl": null,
  "liteUser": null,
  "research": null,
  "restrictTradeSubscription": null,
  "showEUCostReport": null,
  "showGFIS": null,
  "showImpactDashboard": null,
  "showTaxOpt": null,
  "showUkUserLabels": null,
  "showWebNews": null,
  "sideBySide": null,
  "snapshotRefreshTimeout": 30,
} satisfies UserAccountsResponseAllowFeatures

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as UserAccountsResponseAllowFeatures
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


