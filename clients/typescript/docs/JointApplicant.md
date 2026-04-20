
# JointApplicant


## Properties

Name | Type
------------ | -------------
`accreditedInvestorInformation` | [AccreditedInvestorInformation](AccreditedInvestorInformation.md)
`financialInformation` | [Array&lt;FinancialInformation&gt;](FinancialInformation.md)
`firstHolderDetails` | [Array&lt;AssociatedIndividual&gt;](AssociatedIndividual.md)
`regulatedMemberships` | [Array&lt;RegulatedMembership&gt;](RegulatedMembership.md)
`regulatoryInformation` | [Array&lt;RegulatoryInformation&gt;](RegulatoryInformation.md)
`secondHolderDetails` | [Array&lt;AssociatedIndividual&gt;](AssociatedIndividual.md)
`taxInformation` | [IndividualTaxInformation](IndividualTaxInformation.md)
`type` | string
`withholdingStatement` | [WithholdingStatementType](WithholdingStatementType.md)

## Example

```typescript
import type { JointApplicant } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accreditedInvestorInformation": null,
  "financialInformation": null,
  "firstHolderDetails": null,
  "regulatedMemberships": null,
  "regulatoryInformation": null,
  "secondHolderDetails": null,
  "taxInformation": null,
  "type": null,
  "withholdingStatement": null,
} satisfies JointApplicant

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as JointApplicant
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


