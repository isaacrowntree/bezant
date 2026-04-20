
# InformationChange


## Properties

Name | Type
------------ | -------------
`addEntities` | [Array&lt;AddEntity&gt;](AddEntity.md)
`deleteEntities` | [Array&lt;DeleteEntity&gt;](DeleteEntity.md)
`ibAccountId` | string
`updateEntities` | [Array&lt;UpdateEntity&gt;](UpdateEntity.md)

## Example

```typescript
import type { InformationChange } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "addEntities": null,
  "deleteEntities": null,
  "ibAccountId": null,
  "updateEntities": null,
} satisfies InformationChange

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as InformationChange
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


