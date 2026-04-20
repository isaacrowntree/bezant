
# OrganizationIdentification


## Properties

Name | Type
------------ | -------------
`businessDescription` | string
`formationCountry` | string
`formationState` | string
`identification` | string
`identificationCountry` | string
`mailingAddress` | [Address](Address.md)
`name` | string
`phones` | [Array&lt;PhoneInfo&gt;](PhoneInfo.md)
`placeOfBusinessAddress` | [Address](Address.md)
`sameMailAddress` | boolean
`translated` | boolean
`websiteAddress` | string

## Example

```typescript
import type { OrganizationIdentification } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "businessDescription": null,
  "formationCountry": null,
  "formationState": null,
  "identification": null,
  "identificationCountry": null,
  "mailingAddress": null,
  "name": null,
  "phones": null,
  "placeOfBusinessAddress": null,
  "sameMailAddress": null,
  "translated": null,
  "websiteAddress": null,
} satisfies OrganizationIdentification

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as OrganizationIdentification
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


