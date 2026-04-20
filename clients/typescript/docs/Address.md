
# Address


## Properties

Name | Type
------------ | -------------
`city` | string
`country` | string
`postalCode` | string
`state` | string
`street1` | string
`street2` | string

## Example

```typescript
import type { Address } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "city": null,
  "country": null,
  "postalCode": null,
  "state": null,
  "street1": null,
  "street2": null,
} satisfies Address

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Address
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


