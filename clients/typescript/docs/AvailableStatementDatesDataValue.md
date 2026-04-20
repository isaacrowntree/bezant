
# AvailableStatementDatesDataValue


## Properties

Name | Type
------------ | -------------
`annual` | Array&lt;string&gt;
`daily` | [AvailableStatementDatesDataValueDaily](AvailableStatementDatesDataValueDaily.md)
`monthly` | Array&lt;string&gt;

## Example

```typescript
import type { AvailableStatementDatesDataValue } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "annual": null,
  "daily": null,
  "monthly": null,
} satisfies AvailableStatementDatesDataValue

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AvailableStatementDatesDataValue
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


