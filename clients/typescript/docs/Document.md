
# Document


## Properties

Name | Type
------------ | -------------
`attachedFile` | [AttachedFileType](AttachedFileType.md)
`documentType` | string
`execLoginTimestamp` | number
`execTimestamp` | number
`expirationDate` | Date
`externalAccountId` | string
`externalIndividualId` | string
`formNumber` | number
`payload` | [FilePayload](FilePayload.md)
`proofOfAddressType` | string
`proofOfIdentityType` | string
`signature` | string
`signedBy` | Array&lt;string&gt;
`validAddress` | boolean

## Example

```typescript
import type { Document } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "attachedFile": null,
  "documentType": Brokerage Statement,
  "execLoginTimestamp": -1186971863,
  "execTimestamp": -1186971863,
  "expirationDate": null,
  "externalAccountId": null,
  "externalIndividualId": Test_22,
  "formNumber": null,
  "payload": null,
  "proofOfAddressType": Bank Statement,
  "proofOfIdentityType": Driver License,
  "signature": null,
  "signedBy": Jane L Doe,
  "validAddress": null,
} satisfies Document

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Document
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


