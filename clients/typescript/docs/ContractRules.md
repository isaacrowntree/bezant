
# ContractRules

detailed contract information

## Properties

Name | Type
------------ | -------------
`algoEligible` | boolean
`allOrNoneEligible` | boolean
`canTradeAcctIds` | Array&lt;string&gt;
`cashCcy` | string
`cashQtyIncr` | number
`cashSize` | number
`costReport` | boolean
`cqtTypes` | Array&lt;string&gt;
`defaultSize` | number
`displaySize` | number
`error` | string
`forceOrderPreview` | boolean
`fraqInt` | number
`fraqTypes` | Array&lt;string&gt;
`hasSecondary` | boolean
`ibAlgoTypes` | Array&lt;string&gt;
`increment` | number
`incrementDigits` | number
`incrementRules` | [Array&lt;ContractRulesIncrementRulesInner&gt;](ContractRulesIncrementRulesInner.md)
`incrementType` | number
`limitPrice` | number
`modTypes` | Array&lt;any&gt;
`negativeCapable` | boolean
`orderDefaults` | [ContractRulesOrderDefaults](ContractRulesOrderDefaults.md)
`orderOrigination` | string
`orderTypes` | Array&lt;string&gt;
`orderTypesOutside` | Array&lt;string&gt;
`preview` | boolean
`priceMagnifier` | number
`sizeIncrement` | number
`stopPrice` | number
`tifDefaults` | [ContractRulesTifDefaults](ContractRulesTifDefaults.md)
`tifTypes` | Array&lt;string&gt;

## Example

```typescript
import type { ContractRules } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "algoEligible": null,
  "allOrNoneEligible": null,
  "canTradeAcctIds": null,
  "cashCcy": null,
  "cashQtyIncr": null,
  "cashSize": null,
  "costReport": null,
  "cqtTypes": null,
  "defaultSize": null,
  "displaySize": null,
  "error": null,
  "forceOrderPreview": null,
  "fraqInt": null,
  "fraqTypes": null,
  "hasSecondary": null,
  "ibAlgoTypes": null,
  "increment": null,
  "incrementDigits": null,
  "incrementRules": null,
  "incrementType": null,
  "limitPrice": null,
  "modTypes": null,
  "negativeCapable": null,
  "orderDefaults": null,
  "orderOrigination": null,
  "orderTypes": null,
  "orderTypesOutside": null,
  "preview": null,
  "priceMagnifier": null,
  "sizeIncrement": null,
  "stopPrice": null,
  "tifDefaults": null,
  "tifTypes": null,
} satisfies ContractRules

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ContractRules
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


