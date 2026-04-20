
# PhoneInfo


## Properties

Name | Type
------------ | -------------
`country` | string
`number` | string
`type` | string
`verified` | boolean

## Example

```typescript
import type { PhoneInfo } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "country": null,
  "number": null,
  "type": null,
  "verified": null,
} satisfies PhoneInfo

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as PhoneInfo
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


