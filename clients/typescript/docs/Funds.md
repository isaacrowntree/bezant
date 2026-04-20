
# Funds

Contains commodities specific fund values.

## Properties

Name | Type
------------ | -------------
`Lk_Ahd_Avlbl_Fnds` | string
`Prdctd_Pst_xpry_Excss` | string
`current_available` | string
`current_excess` | string
`overnight_available` | string
`overnight_excess` | string

## Example

```typescript
import type { Funds } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "Lk_Ahd_Avlbl_Fnds": null,
  "Prdctd_Pst_xpry_Excss": null,
  "current_available": null,
  "current_excess": null,
  "overnight_available": null,
  "overnight_excess": null,
} satisfies Funds

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Funds
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


