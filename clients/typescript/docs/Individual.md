
# Individual


## Properties

Name | Type
------------ | -------------
`authorizedToSignOnBehalfOfOwner` | boolean
`authorizedTrader` | boolean
`birthName` | [IndividualName](IndividualName.md)
`cityOfBirth` | string
`countryOfBirth` | string
`crs` | [FormCRS](FormCRS.md)
`dateOfBirth` | string
`email` | string
`employeeTitle` | string
`employmentDetails` | [EmploymentDetails](EmploymentDetails.md)
`employmentType` | string
`externalId` | string
`gender` | string
`id` | string
`identification` | [Identification](Identification.md)
`mailingAddress` | [Address](Address.md)
`maritalStatus` | string
`motherMaidenName` | [IndividualName](IndividualName.md)
`name` | [IndividualName](IndividualName.md)
`nativeName` | [IndividualName](IndividualName.md)
`numDependents` | number
`phones` | [Array&lt;PhoneInfo&gt;](PhoneInfo.md)
`primaryTrustee` | boolean
`prohibitedCountryQuestionnaire` | [ProhibitedCountryQuestionnaireList](ProhibitedCountryQuestionnaireList.md)
`residenceAddress` | [ResidenceAddress](ResidenceAddress.md)
`sameMailAddress` | boolean
`taxResidencies` | [Array&lt;TaxResidency&gt;](TaxResidency.md)
`translated` | boolean
`usTaxResident` | boolean
`userId` | string
`w8Ben` | [FormW8BEN](FormW8BEN.md)
`w9` | [FormW9](FormW9.md)

## Example

```typescript
import type { Individual } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "authorizedToSignOnBehalfOfOwner": null,
  "authorizedTrader": null,
  "birthName": null,
  "cityOfBirth": null,
  "countryOfBirth": null,
  "crs": null,
  "dateOfBirth": 1990-08-14,
  "email": null,
  "employeeTitle": null,
  "employmentDetails": null,
  "employmentType": null,
  "externalId": null,
  "gender": null,
  "id": null,
  "identification": null,
  "mailingAddress": null,
  "maritalStatus": null,
  "motherMaidenName": null,
  "name": null,
  "nativeName": null,
  "numDependents": null,
  "phones": null,
  "primaryTrustee": null,
  "prohibitedCountryQuestionnaire": null,
  "residenceAddress": null,
  "sameMailAddress": null,
  "taxResidencies": null,
  "translated": null,
  "usTaxResident": null,
  "userId": null,
  "w8Ben": null,
  "w9": null,
} satisfies Individual

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Individual
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


