
# IserverScannerRunResponse


## Properties

Name | Type
------------ | -------------
`contracts` | [Array&lt;IserverScannerRunResponseContractsInner&gt;](IserverScannerRunResponseContractsInner.md)
`scan_data_column_name` | string

## Example

```typescript
import type { IserverScannerRunResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "contracts": null,
  "scan_data_column_name": null,
} satisfies IserverScannerRunResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as IserverScannerRunResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


