
# Account


## Properties

Name | Type
------------ | -------------
`accountConfiguration` | [AccountConfigurationType](AccountConfigurationType.md)
`accountRep` | [AccountRep](AccountRep.md)
`accountType` | string
`advisorWrapFees` | [AdvisorWrapFeesType](AdvisorWrapFeesType.md)
`alias` | string
`allExchangeAccess` | [Array&lt;ExchangeAccess&gt;](ExchangeAccess.md)
`baseCurrency` | string
`brokerageServiceCodes` | Array&lt;string&gt;
`capabilities` | Array&lt;string&gt;
`clientActiveTrading` | boolean
`clientCommissionSchedule` | [CommissionScheduleType](CommissionScheduleType.md)
`clientInterestMarkupSchedules` | [Array&lt;InterestMarkupType&gt;](InterestMarkupType.md)
`commissionConfigs` | [Array&lt;CommissionConfig&gt;](CommissionConfig.md)
`custodian` | [CustodianType](CustodianType.md)
`decendent` | [IRADecedent](IRADecedent.md)
`depositNotification` | [DepositNotification](DepositNotification.md)
`drip` | boolean
`duplicate` | boolean
`dvpInstructions` | [Array&lt;DVPInstruction&gt;](DVPInstruction.md)
`employeePlan` | string
`extPositionsTransfers` | [Array&lt;ExtPositionsTransferType&gt;](ExtPositionsTransferType.md)
`externalId` | string
`feesTemplateName` | string
`id` | string
`investmentObjectives` | Array&lt;string&gt;
`ira` | boolean
`iraBeneficiaries` | [IRABeneficiariesType](IRABeneficiariesType.md)
`iraOfficialTitle` | string
`iraType` | string
`margin` | string
`migration` | boolean
`multiCurrency` | boolean
`numberOfDuplicates` | number
`optionLevel` | number
`propertyProfile` | string
`sourceAccountId` | string
`stockYieldProgram` | boolean
`successorCustodian` | [CustodianType](CustodianType.md)
`tradingLimits` | [TradingLimits](TradingLimits.md)
`tradingPermissions` | [Array&lt;TradingPermission&gt;](TradingPermission.md)

## Example

```typescript
import type { Account } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountConfiguration": null,
  "accountRep": null,
  "accountType": null,
  "advisorWrapFees": null,
  "alias": null,
  "allExchangeAccess": null,
  "baseCurrency": null,
  "brokerageServiceCodes": null,
  "capabilities": null,
  "clientActiveTrading": null,
  "clientCommissionSchedule": null,
  "clientInterestMarkupSchedules": null,
  "commissionConfigs": null,
  "custodian": null,
  "decendent": null,
  "depositNotification": null,
  "drip": null,
  "duplicate": null,
  "dvpInstructions": null,
  "employeePlan": null,
  "extPositionsTransfers": null,
  "externalId": null,
  "feesTemplateName": null,
  "id": null,
  "investmentObjectives": null,
  "ira": null,
  "iraBeneficiaries": null,
  "iraOfficialTitle": null,
  "iraType": null,
  "margin": null,
  "migration": null,
  "multiCurrency": null,
  "numberOfDuplicates": null,
  "optionLevel": null,
  "propertyProfile": null,
  "sourceAccountId": null,
  "stockYieldProgram": null,
  "successorCustodian": null,
  "tradingLimits": null,
  "tradingPermissions": null,
} satisfies Account

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Account
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


