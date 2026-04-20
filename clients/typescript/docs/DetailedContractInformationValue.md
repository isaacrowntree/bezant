
# DetailedContractInformationValue

Contains the relevant performance data for the specified accountId.

## Properties

Name | Type
------------ | -------------
`baseCurrency` | string
`end` | string
`lastSuccessfulUpdate` | string
`periods` | Array&lt;string&gt;
`start` | string

## Example

```typescript
import type { DetailedContractInformationValue } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "baseCurrency": null,
  "end": null,
  "lastSuccessfulUpdate": null,
  "periods": null,
  "start": null,
} satisfies DetailedContractInformationValue

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as DetailedContractInformationValue
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


