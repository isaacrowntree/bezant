
# WithholdingStatementType


## Properties

Name | Type
------------ | -------------
`accountId` | string
`corporation` | boolean
`dividendRate` | number
`eciRate` | number
`effectiveDate` | Date
`fatcaCompliantType` | string
`flowThrough` | boolean
`interestRate` | number
`treatyCountry` | string
`usBackupWithholding` | boolean
`usOtherRate` | number

## Example

```typescript
import type { WithholdingStatementType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": null,
  "corporation": null,
  "dividendRate": null,
  "eciRate": null,
  "effectiveDate": null,
  "fatcaCompliantType": null,
  "flowThrough": null,
  "interestRate": null,
  "treatyCountry": null,
  "usBackupWithholding": null,
  "usOtherRate": null,
} satisfies WithholdingStatementType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as WithholdingStatementType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


