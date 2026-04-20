
# FinancialInformation


## Properties

Name | Type
------------ | -------------
`additionalSourcesOfIncome` | [Array&lt;SourceOfIncomeType&gt;](SourceOfIncomeType.md)
`annualNetIncome` | number
`investmentExperience` | [Array&lt;AssetExperience&gt;](AssetExperience.md)
`investmentObjectives` | Array&lt;string&gt;
`liquidNetWorth` | number
`netWorth` | number
`questionnaires` | [Array&lt;QuestionnaireType&gt;](QuestionnaireType.md)
`soiQuestionnaire` | [SOIQuestionnaire](SOIQuestionnaire.md)
`sourceOfFunds` | string
`sourcesOfWealth` | [Array&lt;SourceOfWealthType&gt;](SourceOfWealthType.md)
`totalAssets` | number
`translated` | boolean

## Example

```typescript
import type { FinancialInformation } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "additionalSourcesOfIncome": null,
  "annualNetIncome": null,
  "investmentExperience": null,
  "investmentObjectives": null,
  "liquidNetWorth": null,
  "netWorth": null,
  "questionnaires": null,
  "soiQuestionnaire": null,
  "sourceOfFunds": null,
  "sourcesOfWealth": null,
  "totalAssets": null,
  "translated": null,
} satisfies FinancialInformation

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as FinancialInformation
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


