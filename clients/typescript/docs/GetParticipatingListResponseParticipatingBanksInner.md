
# GetParticipatingListResponseParticipatingBanksInner


## Properties

Name | Type
------------ | -------------
`BIC` | string
`clearingCode` | string
`institutionName` | string

## Example

```typescript
import type { GetParticipatingListResponseParticipatingBanksInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "BIC": WEDIHKHHXXX,
  "clearingCode": 390,
  "institutionName": WELAB BANK LIMITED,
} satisfies GetParticipatingListResponseParticipatingBanksInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GetParticipatingListResponseParticipatingBanksInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


