
# GetInvestedAccountsSummaryAccountListInner


## Properties

Name | Type
------------ | -------------
`account` | string
`accountModelNlv` | string
`accountOtherModelsNlv` | string
`alias` | string
`baseCcyAccount` | string
`baseCcyAccountPrecision` | string
`cashInIndependentNlv` | string
`exchangeRate` | number
`nlv` | string
`positionsInIndependentNlv` | string

## Example

```typescript
import type { GetInvestedAccountsSummaryAccountListInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "account": null,
  "accountModelNlv": null,
  "accountOtherModelsNlv": null,
  "alias": null,
  "baseCcyAccount": null,
  "baseCcyAccountPrecision": null,
  "cashInIndependentNlv": null,
  "exchangeRate": null,
  "nlv": null,
  "positionsInIndependentNlv": null,
} satisfies GetInvestedAccountsSummaryAccountListInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GetInvestedAccountsSummaryAccountListInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


