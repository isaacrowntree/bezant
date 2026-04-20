
# UserAccountsResponse


## Properties

Name | Type
------------ | -------------
`accounts` | Array&lt;string&gt;
`acctProps` | [UserAccountsResponseAcctProps](UserAccountsResponseAcctProps.md)
`aliases` | [UserAccountsResponseAliases](UserAccountsResponseAliases.md)
`allowFeatures` | [UserAccountsResponseAllowFeatures](UserAccountsResponseAllowFeatures.md)
`chartPeriods` | [UserAccountsResponseChartPeriods](UserAccountsResponseChartPeriods.md)
`groups` | Array&lt;string&gt;
`isFt` | boolean
`isPaper` | boolean
`profiles` | Array&lt;string&gt;
`selectedAccount` | string
`serverInfo` | [UserAccountsResponseServerInfo](UserAccountsResponseServerInfo.md)
`sessionId` | string

## Example

```typescript
import type { UserAccountsResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accounts": null,
  "acctProps": null,
  "aliases": null,
  "allowFeatures": null,
  "chartPeriods": null,
  "groups": null,
  "isFt": null,
  "isPaper": null,
  "profiles": null,
  "selectedAccount": null,
  "serverInfo": null,
  "sessionId": null,
} satisfies UserAccountsResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as UserAccountsResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


