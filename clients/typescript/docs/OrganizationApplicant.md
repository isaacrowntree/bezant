
# OrganizationApplicant


## Properties

Name | Type
------------ | -------------
`acceptedPrimaryWithholding` | boolean
`accountSupport` | [AccountSupportType](AccountSupportType.md)
`accreditedInvestorInformation` | [AccreditedInvestorInformation](AccreditedInvestorInformation.md)
`associatedEntities` | [AssociatedEntities](AssociatedEntities.md)
`assumedPrimaryReporting` | boolean
`financialInformation` | [Array&lt;FinancialInformation&gt;](FinancialInformation.md)
`identifications` | [Array&lt;OrganizationIdentification&gt;](OrganizationIdentification.md)
`managingOwner` | [ManagingOwner](ManagingOwner.md)
`orgUsSubsidiary` | boolean
`qualifiedIntermediary` | boolean
`regulatedMemberships` | [Array&lt;RegulatedMembership&gt;](RegulatedMembership.md)
`regulatoryInformation` | [Array&lt;RegulatoryInformation&gt;](RegulatoryInformation.md)
`taxResidencies` | [Array&lt;TaxResidency&gt;](TaxResidency.md)
`type` | string
`typeOfTrading` | string
`usTaxPurposeType` | string
`w8BenE` | [FormW8BENE](FormW8BENE.md)
`w8IMY` | [FormW8IMY](FormW8IMY.md)
`withholdingStatement` | [WithholdingStatementType](WithholdingStatementType.md)

## Example

```typescript
import type { OrganizationApplicant } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "acceptedPrimaryWithholding": null,
  "accountSupport": null,
  "accreditedInvestorInformation": null,
  "associatedEntities": null,
  "assumedPrimaryReporting": null,
  "financialInformation": null,
  "identifications": null,
  "managingOwner": null,
  "orgUsSubsidiary": null,
  "qualifiedIntermediary": null,
  "regulatedMemberships": null,
  "regulatoryInformation": null,
  "taxResidencies": null,
  "type": null,
  "typeOfTrading": null,
  "usTaxPurposeType": null,
  "w8BenE": null,
  "w8IMY": null,
  "withholdingStatement": null,
} satisfies OrganizationApplicant

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as OrganizationApplicant
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


