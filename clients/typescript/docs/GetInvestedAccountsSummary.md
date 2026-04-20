
# GetInvestedAccountsSummary


## Properties

Name | Type
------------ | -------------
`accountList` | [Array&lt;GetInvestedAccountsSummaryAccountListInner&gt;](GetInvestedAccountsSummaryAccountListInner.md)
`baseCcyMaster` | string
`baseCcyMasterPrecision` | string
`model` | string
`reqID` | number
`subscriptionStatus` | number

## Example

```typescript
import type { GetInvestedAccountsSummary } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountList": null,
  "baseCcyMaster": null,
  "baseCcyMasterPrecision": null,
  "model": null,
  "reqID": null,
  "subscriptionStatus": null,
} satisfies GetInvestedAccountsSummary

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GetInvestedAccountsSummary
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


