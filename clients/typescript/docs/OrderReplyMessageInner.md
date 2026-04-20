
# OrderReplyMessageInner

An object containing order reply messages emitted against a single order ticket.

## Properties

Name | Type
------------ | -------------
`id` | string
`isSuppressed` | boolean
`message` | Array&lt;string&gt;
`messageIds` | Array&lt;string&gt;

## Example

```typescript
import type { OrderReplyMessageInner } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "id": null,
  "isSuppressed": null,
  "message": null,
  "messageIds": null,
} satisfies OrderReplyMessageInner

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as OrderReplyMessageInner
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


