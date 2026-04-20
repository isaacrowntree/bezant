
# ContraBrokerInfo


## Properties

Name | Type
------------ | -------------
`accountTitle` | string
`accountType` | string
`brokerAccountId` | string
`brokerName` | string
`contactEmail` | string
`contactName` | string
`contactPhone` | string
`country` | string
`depositoryId` | string

## Example

```typescript
import type { ContraBrokerInfo } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "accountTitle": Equatex U.S. Inc. For the Benefit of the Plan Participants of UBS AG,
  "accountType": ORG,
  "brokerAccountId": as3456567678578N,
  "brokerName": JP MORGAN,
  "contactEmail": a@gmail.com,
  "contactName": as,
  "contactPhone": 2039126155,
  "country": United States,
  "depositoryId": 1234,
} satisfies ContraBrokerInfo

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ContraBrokerInfo
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


