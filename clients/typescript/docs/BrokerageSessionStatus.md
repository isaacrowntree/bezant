
# BrokerageSessionStatus


## Properties

Name | Type
------------ | -------------
`MAC` | string
`authenticated` | boolean
`competing` | boolean
`connected` | boolean
`established` | boolean
`fail` | string
`hardware_info` | string
`message` | string
`serverInfo` | [BrokerageSessionStatusServerInfo](BrokerageSessionStatusServerInfo.md)

## Example

```typescript
import type { BrokerageSessionStatus } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "MAC": null,
  "authenticated": null,
  "competing": null,
  "connected": null,
  "established": null,
  "fail": null,
  "hardware_info": null,
  "message": null,
  "serverInfo": null,
} satisfies BrokerageSessionStatus

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as BrokerageSessionStatus
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


