
# TrustApplicant


## Properties

Name | Type
------------ | -------------
`accreditedInvestorInformation` | [AccreditedInvestorInformation](AccreditedInvestorInformation.md)
`beneficiaries` | [AssociationTypeEntities](AssociationTypeEntities.md)
`financialInformation` | [Array&lt;FinancialInformation&gt;](FinancialInformation.md)
`grantors` | [AssociationTypeEntities](AssociationTypeEntities.md)
`identification` | [Array&lt;TrustIdentification&gt;](TrustIdentification.md)
`regulatedMemberships` | [Array&lt;RegulatedMembership&gt;](RegulatedMembership.md)
`regulatoryInformation` | [Array&lt;RegulatoryInformation&gt;](RegulatoryInformation.md)
`taxResidencies` | [Array&lt;TaxResidency&gt;](TaxResidency.md)
`thirdPartyManagement` | boolean
`trustType` | string
`trustees` | [TrusteesType](TrusteesType.md)
`w8BenE` | [FormW8BENE](FormW8BENE.md)
`w8IMY` | [FormW8IMY](FormW8IMY.md)
`withholdingStatement` | [WithholdingStatementType](WithholdingStatementType.md)

## Example

```typescript
import type { TrustApplicant } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accreditedInvestorInformation": null,
  "beneficiaries": null,
  "financialInformation": null,
  "grantors": null,
  "identification": null,
  "regulatedMemberships": null,
  "regulatoryInformation": null,
  "taxResidencies": null,
  "thirdPartyManagement": null,
  "trustType": null,
  "trustees": null,
  "w8BenE": null,
  "w8IMY": null,
  "withholdingStatement": null,
} satisfies TrustApplicant

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TrustApplicant
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


