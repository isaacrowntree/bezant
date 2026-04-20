
# AvailableFundsResponseSecurities

Contains an overview of Security specific fund values.

## Properties

Name | Type
------------ | -------------
`Lk_Ahd_Avlbl_Fnds` | string
`Lk_Ahd_Excss_Lqdty` | string
`Prdctd_Pst_xpry_Excss` | string
`SMA` | string
`current_available` | string
`current_excess` | string
`leverage` | string
`overnight_available` | string
`overnight_excess` | string

## Example

```typescript
import type { AvailableFundsResponseSecurities } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "Lk_Ahd_Avlbl_Fnds": null,
  "Lk_Ahd_Excss_Lqdty": null,
  "Prdctd_Pst_xpry_Excss": null,
  "SMA": null,
  "current_available": null,
  "current_excess": null,
  "leverage": null,
  "overnight_available": null,
  "overnight_excess": null,
} satisfies AvailableFundsResponseSecurities

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AvailableFundsResponseSecurities
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


