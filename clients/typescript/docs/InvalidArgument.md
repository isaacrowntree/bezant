
# InvalidArgument


## Properties

Name | Type
------------ | -------------
`description` | string
`field` | string

## Example

```typescript
import type { InvalidArgument } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "description": Missing required parameter,
  "field": payload,
} satisfies InvalidArgument

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as InvalidArgument
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


