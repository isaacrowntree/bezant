
# ORGRegulatoryInfoType


## Properties

Name | Type
------------ | -------------
`orgRegulators` | [Array&lt;ORGRegulatorType&gt;](ORGRegulatorType.md)
`_public` | boolean
`publicCompanyInfo` | [PublicCompanyInfoType](PublicCompanyInfoType.md)
`regulated` | boolean

## Example

```typescript
import type { ORGRegulatoryInfoType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "orgRegulators": null,
  "_public": null,
  "publicCompanyInfo": null,
  "regulated": null,
} satisfies ORGRegulatoryInfoType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ORGRegulatoryInfoType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


