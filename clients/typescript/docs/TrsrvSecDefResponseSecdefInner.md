
# TrsrvSecDefResponseSecdefInner


## Properties

Name | Type
------------ | -------------
`allExchanges` | string
`assetClass` | string
`chineseName` | string
`conid` | number
`countryCode` | string
`currency` | string
`displayRule` | [Array&lt;TrsrvSecDefResponseSecdefInnerDisplayRuleInner&gt;](TrsrvSecDefResponseSecdefInnerDisplayRuleInner.md)
`expiry` | string
`fullName` | string
`group` | string
`hasOptions` | boolean
`incrementRules` | [Array&lt;TrsrvSecDefResponseSecdefInnerIncrementRulesInner&gt;](TrsrvSecDefResponseSecdefInnerIncrementRulesInner.md)
`isEventContract` | boolean
`isUS` | boolean
`lastTradingDay` | string
`listingExchange` | string
`multiplier` | number
`name` | string
`pageSize` | number
`putOrCall` | string
`sector` | string
`sectorGroup` | string
`strike` | string
`ticker` | string
`time` | number
`type` | string
`undConid` | number

## Example

```typescript
import type { TrsrvSecDefResponseSecdefInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "allExchanges": null,
  "assetClass": null,
  "chineseName": null,
  "conid": null,
  "countryCode": null,
  "currency": null,
  "displayRule": null,
  "expiry": null,
  "fullName": null,
  "group": null,
  "hasOptions": null,
  "incrementRules": null,
  "isEventContract": null,
  "isUS": null,
  "lastTradingDay": null,
  "listingExchange": null,
  "multiplier": null,
  "name": null,
  "pageSize": null,
  "putOrCall": null,
  "sector": null,
  "sectorGroup": null,
  "strike": null,
  "ticker": null,
  "time": null,
  "type": null,
  "undConid": null,
} satisfies TrsrvSecDefResponseSecdefInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TrsrvSecDefResponseSecdefInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


