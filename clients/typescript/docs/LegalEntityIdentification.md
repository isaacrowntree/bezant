
# LegalEntityIdentification


## Properties

Name | Type
------------ | -------------
`exchangeCode` | string
`exchangeSymbol` | string
`formationCountry` | string
`formationType` | string
`identification` | string
`identificationCountry` | string
`mailingAddress` | [Address](Address.md)
`placeOfBusinessAddress` | [Address](Address.md)
`sameMailAddress` | boolean

## Example

```typescript
import type { LegalEntityIdentification } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "exchangeCode": null,
  "exchangeSymbol": null,
  "formationCountry": null,
  "formationType": null,
  "identification": null,
  "identificationCountry": null,
  "mailingAddress": null,
  "placeOfBusinessAddress": null,
  "sameMailAddress": null,
} satisfies LegalEntityIdentification

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as LegalEntityIdentification
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


