
# DepositNotification


## Properties

Name | Type
------------ | -------------
`achDetails` | [ACHDetails](ACHDetails.md)
`amount` | number
`checkDetails` | [CheckDetails](CheckDetails.md)
`currency` | string
`ibAccount` | string
`iraDepositDetails` | [IRADepositDetails](IRADepositDetails.md)
`type` | string
`wireDetails` | [WireDetails](WireDetails.md)

## Example

```typescript
import type { DepositNotification } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "achDetails": null,
  "amount": null,
  "checkDetails": null,
  "currency": null,
  "ibAccount": null,
  "iraDepositDetails": null,
  "type": null,
  "wireDetails": null,
} satisfies DepositNotification

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as DepositNotification
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


