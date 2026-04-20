
# IserverScannerRunRequest


## Properties

Name | Type
------------ | -------------
`filter` | [Array&lt;IserverScannerRunRequestFilterInner&gt;](IserverScannerRunRequestFilterInner.md)
`instrument` | string
`location` | string
`type` | string

## Example

```typescript
import type { IserverScannerRunRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "filter": null,
  "instrument": null,
  "location": null,
  "type": null,
} satisfies IserverScannerRunRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as IserverScannerRunRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


