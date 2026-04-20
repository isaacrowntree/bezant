
# DVPInstruction


## Properties

Name | Type
------------ | -------------
`accountID` | string
`accountName` | string
`agentID` | string
`agentName` | string
`assetClass` | string
`brokerCode` | string
`dayDoID` | string
`_default` | boolean
`exchange` | string
`expiry` | Date
`externalAccountID` | string
`externalId` | string
`firmID` | string
`id` | string
`name` | string
`prepayCommission` | boolean
`prepayTax` | boolean
`role` | string
`txGroupCode` | string
`type` | string

## Example

```typescript
import type { DVPInstruction } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountID": null,
  "accountName": null,
  "agentID": null,
  "agentName": null,
  "assetClass": null,
  "brokerCode": null,
  "dayDoID": null,
  "_default": null,
  "exchange": null,
  "expiry": null,
  "externalAccountID": null,
  "externalId": null,
  "firmID": null,
  "id": null,
  "name": null,
  "prepayCommission": null,
  "prepayTax": null,
  "role": null,
  "txGroupCode": null,
  "type": null,
} satisfies DVPInstruction

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as DVPInstruction
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


