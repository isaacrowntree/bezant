
# SuccessfulTickleResponseHmds

Returns connection details for the historical market data server.

## Properties

Name | Type
------------ | -------------
`authStatus` | Array&lt;any&gt;
`error` | string

## Example

```typescript
import type { SuccessfulTickleResponseHmds } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "authStatus": null,
  "error": null,
} satisfies SuccessfulTickleResponseHmds

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SuccessfulTickleResponseHmds
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


