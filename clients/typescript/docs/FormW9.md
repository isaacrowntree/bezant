
# FormW9


## Properties

Name | Type
------------ | -------------
`blankForm` | boolean
`businessName` | string
`cert1` | boolean
`cert2` | boolean
`cert3` | boolean
`cert4` | boolean
`customerType` | string
`localTaxForms` | [Array&lt;LocalTaxForm&gt;](LocalTaxForm.md)
`name` | string
`otherCustomerType` | string
`proprietaryFormNumber` | number
`signatureType` | string
`taxClassification` | string
`taxFormFile` | string
`tin` | string
`tinType` | string

## Example

```typescript
import type { FormW9 } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "blankForm": null,
  "businessName": null,
  "cert1": null,
  "cert2": null,
  "cert3": null,
  "cert4": null,
  "customerType": null,
  "localTaxForms": null,
  "name": null,
  "otherCustomerType": null,
  "proprietaryFormNumber": null,
  "signatureType": null,
  "taxClassification": null,
  "taxFormFile": null,
  "tin": null,
  "tinType": null,
} satisfies FormW9

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as FormW9
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


