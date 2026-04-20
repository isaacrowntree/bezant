
# AssociatedPerson


## Properties

Name | Type
------------ | -------------
`associations` | Set&lt;string&gt;
`commercial` | string
`countryOfBirth` | string
`countryOfCitizenship` | string
`countryOfLegalResidence` | string
`dateOfBirth` | string
`email` | string
`employmentDetails` | { [key: string]: object; }
`employmentType` | string
`entityId` | number
`externalCode` | string
`firstName` | string
`gender` | string
`identityDocuments` | Array&lt;{ [key: string]: string; }&gt;
`lastLogin` | string
`lastName` | string
`mailing` | { [key: string]: string; }
`maritalStatus` | string
`mdSubscriberStatus` | string
`middleInitial` | string
`middleName` | string
`motersMaidenName` | string
`numberOfDependents` | number
`passwordDate` | string
`phones` | { [key: string]: string; }
`residence` | { [key: string]: string; }
`salutation` | string
`securityDevice` | string
`stateOfLegalResidence` | string
`subscribedServices` | Array&lt;{ [key: string]: object; }&gt;
`suffix` | string
`taxTreatyDetails` | Array&lt;{ [key: string]: string; }&gt;
`userStatus` | string
`userStatusTrading` | string
`username` | string

## Example

```typescript
import type { AssociatedPerson } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "associations": null,
  "commercial": null,
  "countryOfBirth": null,
  "countryOfCitizenship": null,
  "countryOfLegalResidence": null,
  "dateOfBirth": null,
  "email": null,
  "employmentDetails": null,
  "employmentType": null,
  "entityId": null,
  "externalCode": null,
  "firstName": null,
  "gender": null,
  "identityDocuments": null,
  "lastLogin": null,
  "lastName": null,
  "mailing": null,
  "maritalStatus": null,
  "mdSubscriberStatus": null,
  "middleInitial": null,
  "middleName": null,
  "motersMaidenName": null,
  "numberOfDependents": null,
  "passwordDate": null,
  "phones": null,
  "residence": null,
  "salutation": null,
  "securityDevice": null,
  "stateOfLegalResidence": null,
  "subscribedServices": null,
  "suffix": null,
  "taxTreatyDetails": null,
  "userStatus": null,
  "userStatusTrading": null,
  "username": null,
} satisfies AssociatedPerson

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AssociatedPerson
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


