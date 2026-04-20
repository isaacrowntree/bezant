
# LegalEntity


## Properties

Name | Type
------------ | -------------
`address` | [Address](Address.md)
`email` | string
`externalId` | string
`id` | string
`legalEntityIdentification` | [LegalEntityIdentification](LegalEntityIdentification.md)
`name` | string
`phones` | [Array&lt;PhoneInfo&gt;](PhoneInfo.md)
`taxResidencies` | [Array&lt;TaxResidency&gt;](TaxResidency.md)
`translated` | boolean
`usTaxResident` | boolean

## Example

```typescript
import type { LegalEntity } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "address": null,
  "email": null,
  "externalId": null,
  "id": null,
  "legalEntityIdentification": null,
  "name": null,
  "phones": null,
  "taxResidencies": null,
  "translated": null,
  "usTaxResident": null,
} satisfies LegalEntity

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as LegalEntity
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


