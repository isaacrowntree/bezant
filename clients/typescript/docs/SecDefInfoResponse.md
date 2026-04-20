
# SecDefInfoResponse


## Properties

Name | Type
------------ | -------------
`companyName` | string
`conid` | number
`currency` | string
`exchange` | string
`listingExchange` | string
`maturityDate` | string
`priceRendering` | string
`right` | string
`secType` | string
`strike` | number
`ticker` | string
`validExchanges` | string

## Example

```typescript
import type { SecDefInfoResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "companyName": null,
  "conid": null,
  "currency": null,
  "exchange": null,
  "listingExchange": null,
  "maturityDate": null,
  "priceRendering": null,
  "right": null,
  "secType": null,
  "strike": null,
  "ticker": null,
  "validExchanges": null,
} satisfies SecDefInfoResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SecDefInfoResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


