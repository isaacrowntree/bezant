
# AccountData


## Properties

Name | Type
------------ | -------------
`accountAlias` | string
`accountId` | string
`accountTitle` | string
`applicantType` | string
`baseCurrency` | string
`businessDescription` | string
`capabilities` | { [key: string]: Set&lt;string&gt;; }
`class_action_program` | string
`clearingStatus` | string
`clearingStatusDescription` | string
`countryOfCorporation` | string
`dateApproved` | string
`dateBegun` | string
`dateClosed` | string
`dateDelinked` | string
`dateFunded` | string
`dateLinked` | string
`dateOpened` | string
`dividendReinvestment` | { [key: string]: boolean; }
`emailAddress` | string
`equity` | number
`externalId` | string
`feeTemplate` | { [key: string]: string; }
`household` | string
`investmentObjectives` | Set&lt;string&gt;
`limitedOptionTrading` | string
`mailing` | { [key: string]: string; }
`mainAccount` | string
`margin` | string
`masterAccountId` | string
`mifidCategory` | string
`mifirStatus` | string
`officialTitle` | string
`optionLevel` | number
`orgType` | string
`primaryUser` | string
`processType` | string
`propertyProfile` | string
`registeredAddress` | { [key: string]: string; }
`riskScore` | number
`signatures` | Array&lt;string&gt;
`sourceAccountId` | string
`stateCode` | string
`stockYieldProgram` | { [key: string]: string; }
`subType` | string
`taxIds` | Array&lt;{ [key: string]: string; }&gt;
`taxTreatyDetails` | Array&lt;{ [key: string]: string; }&gt;
`tradeIntentionType` | string
`trustType` | string
`usTaxPurposeType` | string

## Example

```typescript
import type { AccountData } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountAlias": null,
  "accountId": null,
  "accountTitle": null,
  "applicantType": null,
  "baseCurrency": null,
  "businessDescription": null,
  "capabilities": null,
  "class_action_program": null,
  "clearingStatus": null,
  "clearingStatusDescription": null,
  "countryOfCorporation": null,
  "dateApproved": null,
  "dateBegun": null,
  "dateClosed": null,
  "dateDelinked": null,
  "dateFunded": null,
  "dateLinked": null,
  "dateOpened": null,
  "dividendReinvestment": null,
  "emailAddress": null,
  "equity": null,
  "externalId": null,
  "feeTemplate": null,
  "household": null,
  "investmentObjectives": null,
  "limitedOptionTrading": null,
  "mailing": null,
  "mainAccount": null,
  "margin": null,
  "masterAccountId": null,
  "mifidCategory": null,
  "mifirStatus": null,
  "officialTitle": null,
  "optionLevel": null,
  "orgType": null,
  "primaryUser": null,
  "processType": null,
  "propertyProfile": null,
  "registeredAddress": null,
  "riskScore": null,
  "signatures": null,
  "sourceAccountId": null,
  "stateCode": null,
  "stockYieldProgram": null,
  "subType": null,
  "taxIds": null,
  "taxTreatyDetails": null,
  "tradeIntentionType": null,
  "trustType": null,
  "usTaxPurposeType": null,
} satisfies AccountData

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AccountData
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


