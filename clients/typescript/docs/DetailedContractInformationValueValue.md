
# DetailedContractInformationValueValue

Returns the performance data for the given period value.

## Properties

Name | Type
------------ | -------------
`cps` | Array&lt;number&gt;
`dates` | Array&lt;string&gt;
`freq` | string
`nav` | Array&lt;number&gt;
`startNav` | [DetailedContractInformationValueValueStartNav](DetailedContractInformationValueValueStartNav.md)

## Example

```typescript
import type { DetailedContractInformationValueValue } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "cps": null,
  "dates": null,
  "freq": null,
  "nav": null,
  "startNav": null,
} satisfies DetailedContractInformationValueValue

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as DetailedContractInformationValueValue
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


