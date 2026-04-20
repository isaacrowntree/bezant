
# AccountDetailsResponse


## Properties

Name | Type
------------ | -------------
`account` | [AccountData](AccountData.md)
`associatedEntities` | [Array&lt;AssociatedEntity&gt;](AssociatedEntity.md)
`associatedPersons` | [Array&lt;AssociatedPerson&gt;](AssociatedPerson.md)
`decedents` | Array&lt;{ [key: string]: string; }&gt;
`entityIRABeneficiaries` | [Array&lt;EntityIRABene&gt;](EntityIRABene.md)
`error` | [ErrorResponse](ErrorResponse.md)
`errorDescription` | string
`financialInformation` | { [key: string]: object; }
`hasError` | boolean
`individualIRABeneficiaries` | [Array&lt;IndividualIRABene&gt;](IndividualIRABene.md)
`marketData` | Array&lt;{ [key: string]: string; }&gt;
`restrictions` | [Set&lt;RestrictionInfo&gt;](RestrictionInfo.md)
`sourcesOfWealth` | Array&lt;{ [key: string]: object; }&gt;
`tradeBundles` | Array&lt;string&gt;
`withHoldingStatement` | { [key: string]: string; }

## Example

```typescript
import type { AccountDetailsResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "account": null,
  "associatedEntities": null,
  "associatedPersons": null,
  "decedents": null,
  "entityIRABeneficiaries": null,
  "error": null,
  "errorDescription": null,
  "financialInformation": null,
  "hasError": null,
  "individualIRABeneficiaries": null,
  "marketData": null,
  "restrictions": null,
  "sourcesOfWealth": null,
  "tradeBundles": null,
  "withHoldingStatement": null,
} satisfies AccountDetailsResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AccountDetailsResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


