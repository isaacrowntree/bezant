
# CheckDetails


## Properties

Name | Type
------------ | -------------
`accountNumber` | string
`checkNumber` | string
`routingNumber` | string

## Example

```typescript
import type { CheckDetails } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountNumber": null,
  "checkNumber": null,
  "routingNumber": null,
} satisfies CheckDetails

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as CheckDetails
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


