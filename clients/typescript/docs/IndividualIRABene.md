
# IndividualIRABene


## Properties

Name | Type
------------ | -------------
`dateOfBirth` | string
`firstName` | string
`identification` | { [key: string]: string; }
`lastName` | string
`location` | { [key: string]: string; }
`ownership` | number
`perStripes` | string
`relationship` | string
`type` | string

## Example

```typescript
import type { IndividualIRABene } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "dateOfBirth": null,
  "firstName": null,
  "identification": null,
  "lastName": null,
  "location": null,
  "ownership": null,
  "perStripes": null,
  "relationship": null,
  "type": null,
} satisfies IndividualIRABene

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as IndividualIRABene
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


