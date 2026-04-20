
# Identification

Identification information of the associated person.

## Properties

Name | Type
------------ | -------------
`alienCard` | string
`cardColor` | string
`citizenship` | string
`citizenship2` | string
`citizenship3` | string
`driversLicense` | string
`educationalQualification` | string
`expirationDate` | Date
`expire` | boolean
`fathersName` | string
`greenCard` | boolean
`hkTravelPermit` | string
`issuingCountry` | string
`issuingState` | string
`legalResidenceCountry` | string
`legalResidenceState` | string
`medicareCard` | string
`medicareReference` | string
`nationalCard` | string
`panNumber` | string
`passport` | string
`proofOfAgeCard` | string
`rta` | string
`sin` | string
`ssn` | string
`taxId` | string

## Example

```typescript
import type { Identification } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "alienCard": null,
  "cardColor": null,
  "citizenship": AUS,
  "citizenship2": null,
  "citizenship3": null,
  "driversLicense": 989444798,
  "educationalQualification": null,
  "expirationDate": null,
  "expire": null,
  "fathersName": null,
  "greenCard": null,
  "hkTravelPermit": null,
  "issuingCountry": AUS,
  "issuingState": null,
  "legalResidenceCountry": null,
  "legalResidenceState": null,
  "medicareCard": null,
  "medicareReference": null,
  "nationalCard": null,
  "panNumber": null,
  "passport": null,
  "proofOfAgeCard": null,
  "rta": null,
  "sin": null,
  "ssn": null,
  "taxId": null,
} satisfies Identification

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Identification
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


