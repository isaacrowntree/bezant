
# CommissionMarkupType


## Properties

Name | Type
------------ | -------------
`amount` | number
`code` | string
`maximum` | number
`minimum` | number
`plusCost` | boolean
`stairs` | [Array&lt;MarkupStaircaseType&gt;](MarkupStaircaseType.md)
`ticketCharge` | number
`type` | string

## Example

```typescript
import type { CommissionMarkupType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "amount": null,
  "code": null,
  "maximum": null,
  "minimum": null,
  "plusCost": null,
  "stairs": null,
  "ticketCharge": null,
  "type": null,
} satisfies CommissionMarkupType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as CommissionMarkupType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


