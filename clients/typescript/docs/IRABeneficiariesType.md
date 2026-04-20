
# IRABeneficiariesType


## Properties

Name | Type
------------ | -------------
`contingentBeneficiaries` | [Array&lt;IRAContingentBeneficiary&gt;](IRAContingentBeneficiary.md)
`contingentBeneficiaryEntities` | [Array&lt;IRAContingentBeneficiaryEntity&gt;](IRAContingentBeneficiaryEntity.md)
`primaryBeneficiaries` | [Array&lt;IRAPrimaryBeneficiary&gt;](IRAPrimaryBeneficiary.md)
`primaryBeneficiaryEntities` | [Array&lt;IRAPrimaryBeneficiaryEntity&gt;](IRAPrimaryBeneficiaryEntity.md)
`spousePrimaryBeneficary` | boolean
`successor` | boolean

## Example

```typescript
import type { IRABeneficiariesType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "contingentBeneficiaries": null,
  "contingentBeneficiaryEntities": null,
  "primaryBeneficiaries": null,
  "primaryBeneficiaryEntities": null,
  "spousePrimaryBeneficary": null,
  "successor": null,
} satisfies IRABeneficiariesType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as IRABeneficiariesType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


