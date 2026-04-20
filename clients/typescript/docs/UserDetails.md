
# UserDetails


## Properties

Name | Type
------------ | -------------
`authorizedPerson` | boolean
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
`ownershipPercentage` | number
`phones` | [Array&lt;PhoneInfo&gt;](PhoneInfo.md)
`primaryTrustee` | boolean
`prohibitedCountryQuestionnaire` | [ProhibitedCountryQuestionnaireList](ProhibitedCountryQuestionnaireList.md)
`referenceUsername` | string
`residenceAddress` | [ResidenceAddress](ResidenceAddress.md)
`sameMailAddress` | boolean
`taxResidencies` | [Array&lt;TaxResidency&gt;](TaxResidency.md)
`title` | [Array&lt;Title&gt;](Title.md)
`translated` | boolean
`usTaxResident` | boolean
`userId` | string
`w8Ben` | [FormW8BEN](FormW8BEN.md)
`w9` | [FormW9](FormW9.md)

## Example

```typescript
import type { UserDetails } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "authorizedPerson": null,
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
  "ownershipPercentage": null,
  "phones": null,
  "primaryTrustee": null,
  "prohibitedCountryQuestionnaire": null,
  "referenceUsername": null,
  "residenceAddress": null,
  "sameMailAddress": null,
  "taxResidencies": null,
  "title": null,
  "translated": null,
  "usTaxResident": null,
  "userId": null,
  "w8Ben": null,
  "w9": null,
} satisfies UserDetails

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as UserDetails
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


