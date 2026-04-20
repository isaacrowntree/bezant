
# Subaccounts2Response


## Properties

Name | Type
------------ | -------------
`metadata` | [Subaccounts2ResponseMetadata](Subaccounts2ResponseMetadata.md)
`subaccounts` | [Array&lt;AccountAttributes&gt;](AccountAttributes.md)

## Example

```typescript
import type { Subaccounts2Response } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "metadata": null,
  "subaccounts": null,
} satisfies Subaccounts2Response

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Subaccounts2Response
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


