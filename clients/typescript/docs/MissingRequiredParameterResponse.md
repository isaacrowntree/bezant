
# MissingRequiredParameterResponse


## Properties

Name | Type
------------ | -------------
`invalidArguments` | [Array&lt;InvalidArgument&gt;](InvalidArgument.md)
`status` | number
`title` | string
`type` | string

## Example

```typescript
import type { MissingRequiredParameterResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "invalidArguments": null,
  "status": 400,
  "title": Bad Request,
  "type": /invalid-argument,
} satisfies MissingRequiredParameterResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as MissingRequiredParameterResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


