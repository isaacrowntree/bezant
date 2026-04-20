
# SummaryOfAccountMarginResponseCryptoAtPaxos


## Properties

Name | Type
------------ | -------------
`Prdctd_Pst_xpry_Mrgn__Opn` | string
`Prjctd_Lk_Ahd_Mntnnc_Mrgn` | string
`Prjctd_Ovrnght_Mntnnc_Mrgn` | string
`current_initial` | string
`current_maint` | string
`projected_liquidity_inital_margin` | string
`projected_overnight_initial_margin` | string

## Example

```typescript
import type { SummaryOfAccountMarginResponseCryptoAtPaxos } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "Prdctd_Pst_xpry_Mrgn__Opn": null,
  "Prjctd_Lk_Ahd_Mntnnc_Mrgn": null,
  "Prjctd_Ovrnght_Mntnnc_Mrgn": null,
  "current_initial": null,
  "current_maint": null,
  "projected_liquidity_inital_margin": null,
  "projected_overnight_initial_margin": null,
} satisfies SummaryOfAccountMarginResponseCryptoAtPaxos

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SummaryOfAccountMarginResponseCryptoAtPaxos
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


