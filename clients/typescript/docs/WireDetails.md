
# WireDetails


## Properties

Name | Type
------------ | -------------
`bankAccountNumber` | string
`bankCode` | string
`bankName` | string
`countryCode` | string
`instruction` | string
`referenceNumber` | string
`routingNumber` | string

## Example

```typescript
import type { WireDetails } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "bankAccountNumber": null,
  "bankCode": null,
  "bankName": null,
  "countryCode": null,
  "instruction": null,
  "referenceNumber": null,
  "routingNumber": null,
} satisfies WireDetails

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as WireDetails
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


