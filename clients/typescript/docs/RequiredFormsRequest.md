
# RequiredFormsRequest


## Properties

Name | Type
------------ | -------------
`accountType` | string
`additionalAccount` | boolean
`additionalApplicant` | boolean
`applicantType` | string
`capability` | Array&lt;string&gt;
`faTradingAccount` | boolean
`ira` | boolean
`iraType` | string
`mifidCategory` | string
`processType` | string
`requestedFCBP` | boolean
`requestedSYEP` | boolean
`residenceCountry` | string
`tradingPermissions` | Array&lt;string&gt;

## Example

```typescript
import type { RequiredFormsRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountType": null,
  "additionalAccount": null,
  "additionalApplicant": null,
  "applicantType": null,
  "capability": null,
  "faTradingAccount": null,
  "ira": null,
  "iraType": null,
  "mifidCategory": null,
  "processType": null,
  "requestedFCBP": null,
  "requestedSYEP": null,
  "residenceCountry": null,
  "tradingPermissions": null,
} satisfies RequiredFormsRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as RequiredFormsRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


