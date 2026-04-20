
# Customer


## Properties

Name | Type
------------ | -------------
`accountHolder` | [IndividualApplicant](IndividualApplicant.md)
`directTradingAccess` | boolean
`email` | string
`externalId` | string
`governingState` | string
`id` | string
`independentAccount` | boolean
`jointHolders` | [JointApplicant](JointApplicant.md)
`legalResidenceCountry` | string
`mdStatusNonPro` | boolean
`meetAmlStandard` | string
`meetsAmlStandard` | string
`optForDebitCard` | boolean
`organization` | [OrganizationApplicant](OrganizationApplicant.md)
`originCountry` | string
`paperAccount` | boolean
`preferredPrimaryLanguage` | string
`preferredSecondaryLanguage` | string
`prefix` | string
`roboFaClient` | boolean
`taxTreatyCountry` | string
`terminationAge` | number
`transferUsMicroCapStock` | boolean
`trust` | [TrustApplicant](TrustApplicant.md)
`type` | string
`userName` | string
`userNameAlias` | string
`userNameSource` | string

## Example

```typescript
import type { Customer } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountHolder": null,
  "directTradingAccess": null,
  "email": null,
  "externalId": null,
  "governingState": null,
  "id": null,
  "independentAccount": null,
  "jointHolders": null,
  "legalResidenceCountry": null,
  "mdStatusNonPro": null,
  "meetAmlStandard": null,
  "meetsAmlStandard": null,
  "optForDebitCard": null,
  "organization": null,
  "originCountry": null,
  "paperAccount": null,
  "preferredPrimaryLanguage": null,
  "preferredSecondaryLanguage": null,
  "prefix": null,
  "roboFaClient": null,
  "taxTreatyCountry": null,
  "terminationAge": null,
  "transferUsMicroCapStock": null,
  "trust": null,
  "type": null,
  "userName": null,
  "userNameAlias": null,
  "userNameSource": null,
} satisfies Customer

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Customer
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


