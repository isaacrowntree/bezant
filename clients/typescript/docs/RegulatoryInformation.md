
# RegulatoryInformation


## Properties

Name | Type
------------ | -------------
`affiliationDetails` | [AffiliationDetailsType](AffiliationDetailsType.md)
`ausExposureDetails` | [AUSExposureDetailsType](AUSExposureDetailsType.md)
`controllerExchangeCode` | string
`financialOrgTypes` | Array&lt;string&gt;
`orgRegulatoryInfo` | [ORGRegulatoryInfoType](ORGRegulatoryInfoType.md)
`politicalMilitaryDiplomaticDetails` | [PoliticalMilitaryDiplomaticDetailsType](PoliticalMilitaryDiplomaticDetailsType.md)
`regulatoryDetail` | [Array&lt;RegulatoryDetail&gt;](RegulatoryDetail.md)
`regulatoryDetails` | [Array&lt;RegulatoryDetail&gt;](RegulatoryDetail.md)
`selfRegulatedMembership` | [SelfRegulatedMembershipType](SelfRegulatedMembershipType.md)
`translated` | boolean

## Example

```typescript
import type { RegulatoryInformation } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "affiliationDetails": null,
  "ausExposureDetails": null,
  "controllerExchangeCode": null,
  "financialOrgTypes": null,
  "orgRegulatoryInfo": null,
  "politicalMilitaryDiplomaticDetails": null,
  "regulatoryDetail": null,
  "regulatoryDetails": null,
  "selfRegulatedMembership": null,
  "translated": null,
} satisfies RegulatoryInformation

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as RegulatoryInformation
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


