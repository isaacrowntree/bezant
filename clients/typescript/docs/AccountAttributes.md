
# AccountAttributes


## Properties

Name | Type
------------ | -------------
`PrepaidCrypto_P` | boolean
`PrepaidCrypto_Z` | boolean
`accountAlias` | string
`accountId` | string
`accountStatus` | number
`accountTitle` | string
`accountVan` | string
`acctCustType` | string
`brokerageAccess` | boolean
`businessType` | string
`category` | string
`clearingStatus` | string
`covestor` | boolean
`currency` | string
`desc` | string
`displayName` | string
`faClient` | boolean
`ibEntity` | string
`id` | string
`noClientTrading` | boolean
`parent` | [AccountAttributesParent](AccountAttributesParent.md)
`trackVirtualFXPortfolio` | boolean
`tradingType` | string
`type` | string

## Example

```typescript
import type { AccountAttributes } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "PrepaidCrypto_P": null,
  "PrepaidCrypto_Z": null,
  "accountAlias": null,
  "accountId": null,
  "accountStatus": null,
  "accountTitle": null,
  "accountVan": null,
  "acctCustType": null,
  "brokerageAccess": null,
  "businessType": null,
  "category": null,
  "clearingStatus": null,
  "covestor": null,
  "currency": null,
  "desc": null,
  "displayName": null,
  "faClient": null,
  "ibEntity": null,
  "id": null,
  "noClientTrading": null,
  "parent": null,
  "trackVirtualFXPortfolio": null,
  "tradingType": null,
  "type": null,
} satisfies AccountAttributes

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AccountAttributes
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


