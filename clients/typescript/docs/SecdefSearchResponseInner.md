
# SecdefSearchResponseInner


## Properties

Name | Type
------------ | -------------
`bondid` | number
`companyHeader` | string
`companyName` | string
`conid` | string
`description` | string
`fop` | string
`issuers` | [Array&lt;SecdefSearchResponseInnerIssuersInner&gt;](SecdefSearchResponseInnerIssuersInner.md)
`opt` | string
`restricted` | boolean
`sections` | [Array&lt;SecdefSearchResponseInnerSectionsInner&gt;](SecdefSearchResponseInnerSectionsInner.md)
`symbol` | string
`war` | string

## Example

```typescript
import type { SecdefSearchResponseInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "bondid": null,
  "companyHeader": null,
  "companyName": null,
  "conid": null,
  "description": null,
  "fop": null,
  "issuers": null,
  "opt": null,
  "restricted": null,
  "sections": null,
  "symbol": null,
  "war": null,
} satisfies SecdefSearchResponseInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SecdefSearchResponseInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


