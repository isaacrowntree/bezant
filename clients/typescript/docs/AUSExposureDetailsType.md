
# AUSExposureDetailsType


## Properties

Name | Type
------------ | -------------
`ausExposureRelationship` | string
`licenseNumber` | number
`personName` | string

## Example

```typescript
import type { AUSExposureDetailsType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "ausExposureRelationship": null,
  "licenseNumber": null,
  "personName": null,
} satisfies AUSExposureDetailsType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AUSExposureDetailsType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


