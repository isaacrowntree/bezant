
# AttachedFileType


## Properties

Name | Type
------------ | -------------
`fileLength` | number
`fileName` | string
`sha1Checksum` | string

## Example

```typescript
import type { AttachedFileType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "fileLength": 432177,
  "fileName": Form5001.pdf,
  "sha1Checksum": 03D899BA757F617C907A1F021D7046AC1DAC8707,
} satisfies AttachedFileType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AttachedFileType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


