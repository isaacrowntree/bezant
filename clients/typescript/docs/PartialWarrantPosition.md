
# PartialWarrantPosition


## Properties

Name | Type
------------ | -------------
`all` | boolean
`expirationDate` | string
`numberOfShares` | number
`optionType` | string
`position` | string
`strikePrice` | number
`symbol` | string

## Example

```typescript
import type { PartialWarrantPosition } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "all": null,
  "expirationDate": null,
  "numberOfShares": null,
  "optionType": null,
  "position": null,
  "strikePrice": null,
  "symbol": null,
} satisfies PartialWarrantPosition

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PartialWarrantPosition
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


