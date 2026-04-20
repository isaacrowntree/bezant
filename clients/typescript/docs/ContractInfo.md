
# ContractInfo


## Properties

Name | Type
------------ | -------------
`allow_sell_long` | boolean
`category` | string
`cfi_code` | string
`classifier` | string
`company_name` | string
`con_id` | number
`contract_clarification_type` | string
`contract_month` | string
`currency` | string
`cusip` | string
`exchange` | string
`expiry_full` | string
`industry` | string
`instrument_type` | string
`is_zero_commission_security` | boolean
`local_symbol` | string
`maturity_date` | string
`multiplier` | string
`r_t_h` | boolean
`smart_available` | boolean
`symbol` | string
`text` | string
`trading_class` | string
`underlying_con_id` | number
`underlying_issuer` | string
`valid_exchanges` | string

## Example

```typescript
import type { ContractInfo } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "allow_sell_long": null,
  "category": null,
  "cfi_code": null,
  "classifier": null,
  "company_name": null,
  "con_id": null,
  "contract_clarification_type": null,
  "contract_month": null,
  "currency": null,
  "cusip": null,
  "exchange": null,
  "expiry_full": null,
  "industry": null,
  "instrument_type": null,
  "is_zero_commission_security": null,
  "local_symbol": null,
  "maturity_date": null,
  "multiplier": null,
  "r_t_h": null,
  "smart_available": null,
  "symbol": null,
  "text": null,
  "trading_class": null,
  "underlying_con_id": null,
  "underlying_issuer": null,
  "valid_exchanges": null,
} satisfies ContractInfo

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ContractInfo
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


