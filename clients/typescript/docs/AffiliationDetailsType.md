
# AffiliationDetailsType


## Properties

Name | Type
------------ | -------------
`affiliationRelationship` | string
`company` | string
`companyEmailAddress` | string
`companyId` | number
`companyMailingAddress` | [Address](Address.md)
`companyPhone` | string
`duplicateStmtRequired` | boolean
`personName` | string

## Example

```typescript
import type { AffiliationDetailsType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "affiliationRelationship": null,
  "company": null,
  "companyEmailAddress": null,
  "companyId": null,
  "companyMailingAddress": null,
  "companyPhone": null,
  "duplicateStmtRequired": null,
  "personName": null,
} satisfies AffiliationDetailsType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AffiliationDetailsType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


