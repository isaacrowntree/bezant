
# StmtRequest


## Properties

Name | Type
------------ | -------------
`accountId` | string
`accountIds` | Array&lt;string&gt;
`cryptoConsolIfAvailable` | boolean
`endDate` | string
`gzip` | boolean
`language` | string
`mimeType` | string
`multiAccountFormat` | string
`startDate` | string

## Example

```typescript
import type { StmtRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountId": null,
  "accountIds": [U123, U456],
  "cryptoConsolIfAvailable": null,
  "endDate": null,
  "gzip": null,
  "language": en, fr defaults to en (english),
  "mimeType": application/pdf, text/html, or text/csv,
  "multiAccountFormat": null,
  "startDate": null,
} satisfies StmtRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as StmtRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


