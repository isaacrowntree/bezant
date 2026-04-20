
# EmploymentDetails


## Properties

Name | Type
------------ | -------------
`businessDescription` | string
`description` | string
`emplCountryResCountryDetails` | string
`employer` | string
`employerAddress` | [Address](Address.md)
`employerBusiness` | string
`employerPhone` | string
`occupation` | string

## Example

```typescript
import type { EmploymentDetails } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "businessDescription": null,
  "description": null,
  "emplCountryResCountryDetails": null,
  "employer": null,
  "employerAddress": null,
  "employerBusiness": null,
  "employerPhone": null,
  "occupation": null,
} satisfies EmploymentDetails

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as EmploymentDetails
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


