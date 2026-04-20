
# AlertCreationRequestConditionsInner


## Properties

Name | Type
------------ | -------------
`conidex` | string
`logicBind` | string
`operator` | string
`timeZone` | string
`triggerMethod` | string
`type` | number
`value` | string

## Example

```typescript
import type { AlertCreationRequestConditionsInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "conidex": 265598@SMART,
  "logicBind": null,
  "operator": null,
  "timeZone": null,
  "triggerMethod": null,
  "type": null,
  "value": null,
} satisfies AlertCreationRequestConditionsInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AlertCreationRequestConditionsInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


