
# IserverScannerParams


## Properties

Name | Type
------------ | -------------
`filter_list` | [Array&lt;IserverScannerParamsFilterListInner&gt;](IserverScannerParamsFilterListInner.md)
`instrument_list` | [Array&lt;IserverScannerParamsInstrumentListInner&gt;](IserverScannerParamsInstrumentListInner.md)
`location_tree` | [Array&lt;IserverScannerParamsLocationTreeInner&gt;](IserverScannerParamsLocationTreeInner.md)
`scan_type_list` | [Array&lt;IserverScannerParamsScanTypeListInner&gt;](IserverScannerParamsScanTypeListInner.md)

## Example

```typescript
import type { IserverScannerParams } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "filter_list": null,
  "instrument_list": null,
  "location_tree": null,
  "scan_type_list": null,
} satisfies IserverScannerParams

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as IserverScannerParams
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


