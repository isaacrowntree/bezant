
# LedgerValue

Object describing the account\'s balances in its base currency, by asset class and account segments. Will be duplicated by another object in response bearing the currency\'s name.

## Properties

Name | Type
------------ | -------------
`acctcode` | string
`cashbalance` | number
`cashbalancefxsegment` | number
`commoditymarketvalue` | number
`corporatebondsmarketvalue` | number
`currency` | string
`dividends` | number
`exchangerate` | number
`funds` | number
`futuremarketvalue` | number
`futureoptionmarketvalue` | number
`futuresonlypnl` | number
`interest` | number
`issueroptionsmarketvalue` | number
`key` | string
`moneyfunds` | number
`netliquidationvalue` | number
`realizedpnl` | number
`secondkey` | string
`sessionid` | number
`settledcash` | number
`severity` | number
`stockmarketvalue` | number
`stockoptionmarketvalue` | number
`tbillsmarketvalue` | number
`tbondsmarketvalue` | number
`timestamp` | number
`unrealizedpnl` | number
`warrantsmarketvalue` | number

## Example

```typescript
import type { LedgerValue } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "acctcode": null,
  "cashbalance": null,
  "cashbalancefxsegment": null,
  "commoditymarketvalue": null,
  "corporatebondsmarketvalue": null,
  "currency": null,
  "dividends": null,
  "exchangerate": null,
  "funds": null,
  "futuremarketvalue": null,
  "futureoptionmarketvalue": null,
  "futuresonlypnl": null,
  "interest": null,
  "issueroptionsmarketvalue": null,
  "key": null,
  "moneyfunds": null,
  "netliquidationvalue": null,
  "realizedpnl": null,
  "secondkey": null,
  "sessionid": null,
  "settledcash": null,
  "severity": null,
  "stockmarketvalue": null,
  "stockoptionmarketvalue": null,
  "tbillsmarketvalue": null,
  "tbondsmarketvalue": null,
  "timestamp": null,
  "unrealizedpnl": null,
  "warrantsmarketvalue": null,
} satisfies LedgerValue

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as LedgerValue
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


