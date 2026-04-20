
# FormW8BEN


## Properties

Name | Type
------------ | -------------
`blankForm` | boolean
`cert` | boolean
`electronicFormat` | boolean
`explanation` | string
`foreignTaxId` | string
`localTaxForms` | [Array&lt;LocalTaxForm&gt;](LocalTaxForm.md)
`name` | string
`part29ACountry` | string
`proprietaryFormNumber` | number
`referenceNumber` | number
`signatureType` | string
`submitDate` | string
`taxFormFile` | string
`tin` | string
`tinOrExplanationRequired` | boolean

## Example

```typescript
import type { FormW8BEN } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "blankForm": null,
  "cert": null,
  "electronicFormat": null,
  "explanation": null,
  "foreignTaxId": null,
  "localTaxForms": null,
  "name": null,
  "part29ACountry": null,
  "proprietaryFormNumber": null,
  "referenceNumber": null,
  "signatureType": null,
  "submitDate": null,
  "taxFormFile": null,
  "tin": null,
  "tinOrExplanationRequired": null,
} satisfies FormW8BEN

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as FormW8BEN
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


