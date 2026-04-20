
# TrustIdentification


## Properties

Name | Type
------------ | -------------
`address` | [Address](Address.md)
`dateFormed` | Date
`description` | string
`formationCountry` | string
`formationState` | string
`mailingAddress` | [Address](Address.md)
`name` | string
`phones` | [Array&lt;PhoneInfo&gt;](PhoneInfo.md)
`purposeOfTrust` | string
`registrationCountry` | string
`registrationNumber` | string
`registrationType` | string
`sameMailAddress` | boolean
`translated` | boolean
`typeOfTrust` | string

## Example

```typescript
import type { TrustIdentification } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "address": null,
  "dateFormed": null,
  "description": null,
  "formationCountry": null,
  "formationState": null,
  "mailingAddress": null,
  "name": null,
  "phones": null,
  "purposeOfTrust": null,
  "registrationCountry": null,
  "registrationNumber": null,
  "registrationType": null,
  "sameMailAddress": null,
  "translated": null,
  "typeOfTrust": null,
} satisfies TrustIdentification

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TrustIdentification
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


