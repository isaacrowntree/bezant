
# EntityIRABene


## Properties

Name | Type
------------ | -------------
`articleOfWill` | string
`entityType` | string
`location` | { [key: string]: string; }
`name` | string
`type` | string

## Example

```typescript
import type { EntityIRABene } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "articleOfWill": null,
  "entityType": null,
  "location": null,
  "name": null,
  "type": null,
} satisfies EntityIRABene

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as EntityIRABene
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


