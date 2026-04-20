
# PartialOptionPosition


## Properties

Name | Type
------------ | -------------
`all` | boolean
`expirationDate` | string
`numberOfContracts` | number
`optionType` | string
`position` | string
`strikePrice` | number
`symbol` | string

## Example

```typescript
import type { PartialOptionPosition } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "all": null,
  "expirationDate": null,
  "numberOfContracts": null,
  "optionType": null,
  "position": null,
  "strikePrice": null,
  "symbol": null,
} satisfies PartialOptionPosition

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PartialOptionPosition
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


