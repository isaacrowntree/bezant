
# IndividualApplicant


## Properties

Name | Type
------------ | -------------
`accountHolderDetails` | [Array&lt;AssociatedIndividual&gt;](AssociatedIndividual.md)
`accreditedInvestorInformation` | [AccreditedInvestorInformation](AccreditedInvestorInformation.md)
`associatedIndividual` | [AssociatedIndividual](AssociatedIndividual.md)
`financialInformation` | [Array&lt;FinancialInformation&gt;](FinancialInformation.md)
`regulatedMemberships` | [Array&lt;RegulatedMembership&gt;](RegulatedMembership.md)
`regulatoryInformation` | [Array&lt;RegulatoryInformation&gt;](RegulatoryInformation.md)
`taxInformation` | [IndividualTaxInformation](IndividualTaxInformation.md)
`withholdingStatement` | [WithholdingStatementType](WithholdingStatementType.md)

## Example

```typescript
import type { IndividualApplicant } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountHolderDetails": null,
  "accreditedInvestorInformation": null,
  "associatedIndividual": null,
  "financialInformation": null,
  "regulatedMemberships": null,
  "regulatoryInformation": null,
  "taxInformation": null,
  "withholdingStatement": null,
} satisfies IndividualApplicant

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as IndividualApplicant
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


