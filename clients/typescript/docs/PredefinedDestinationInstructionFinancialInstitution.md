
# PredefinedDestinationInstructionFinancialInstitution


## Properties

Name | Type
------------ | -------------
`branchCode` | string
`branchCodeType` | string
`clientAccountId` | string
`identifier` | string
`identifierType` | string
`name` | string

## Example

```typescript
import type { PredefinedDestinationInstructionFinancialInstitution } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "branchCode": null,
  "branchCodeType": BSB_AUD,
  "clientAccountId": 132456789,
  "identifier": SBIN001000,
  "identifierType": BIC,
  "name": SBI BANK,
} satisfies PredefinedDestinationInstructionFinancialInstitution

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PredefinedDestinationInstructionFinancialInstitution
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


