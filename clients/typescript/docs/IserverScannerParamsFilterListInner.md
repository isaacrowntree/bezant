
# IserverScannerParamsFilterListInner


## Properties

Name | Type
------------ | -------------
`code` | string
`combo_values` | [Array&lt;IserverScannerParamsFilterListInnerComboValuesInner&gt;](IserverScannerParamsFilterListInnerComboValuesInner.md)
`display_name` | string
`group` | string
`type` | string

## Example

```typescript
import type { IserverScannerParamsFilterListInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "code": null,
  "combo_values": null,
  "display_name": null,
  "group": null,
  "type": null,
} satisfies IserverScannerParamsFilterListInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as IserverScannerParamsFilterListInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


