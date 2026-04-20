
# AlgoParam


## Properties

Name | Type
------------ | -------------
`defaultValue` | [AlgoParamDefaultValue](AlgoParamDefaultValue.md)
`description` | string
`guiRank` | number
`id` | string
`legalStrings` | Array&lt;string&gt;
`maxValue` | number
`minValue` | number
`name` | string
`required` | boolean
`valueClassName` | string

## Example

```typescript
import type { AlgoParam } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "defaultValue": null,
  "description": null,
  "guiRank": null,
  "id": null,
  "legalStrings": null,
  "maxValue": null,
  "minValue": null,
  "name": null,
  "required": null,
  "valueClassName": null,
} satisfies AlgoParam

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AlgoParam
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


