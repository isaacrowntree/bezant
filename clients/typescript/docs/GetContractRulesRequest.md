
# GetContractRulesRequest


## Properties

Name | Type
------------ | -------------
`conid` | number
`isBuy` | boolean
`modifyOrder` | boolean
`orderId` | number

## Example

```typescript
import type { GetContractRulesRequest } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "conid": null,
  "isBuy": null,
  "modifyOrder": null,
  "orderId": null,
} satisfies GetContractRulesRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GetContractRulesRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


