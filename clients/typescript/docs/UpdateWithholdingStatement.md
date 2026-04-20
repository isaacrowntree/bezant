
# UpdateWithholdingStatement


## Properties

Name | Type
------------ | -------------
`accountId` | string
`certW8Imy` | boolean
`effectiveDate` | Date
`fatcaCompliantType` | string
`treatyCountry` | string
`usIncomeTax` | boolean

## Example

```typescript
import type { UpdateWithholdingStatement } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": null,
  "certW8Imy": null,
  "effectiveDate": null,
  "fatcaCompliantType": null,
  "treatyCountry": null,
  "usIncomeTax": null,
} satisfies UpdateWithholdingStatement

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as UpdateWithholdingStatement
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


