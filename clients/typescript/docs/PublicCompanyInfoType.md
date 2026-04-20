
# PublicCompanyInfoType


## Properties

Name | Type
------------ | -------------
`exchangeTradedOn` | string
`quotedSymbol` | string

## Example

```typescript
import type { PublicCompanyInfoType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "exchangeTradedOn": null,
  "quotedSymbol": null,
} satisfies PublicCompanyInfoType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PublicCompanyInfoType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


