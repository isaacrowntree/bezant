
# SsoValidateResponse


## Properties

Name | Type
------------ | -------------
`AUTH_TIME` | number
`CREDENTIAL` | string
`EXPIRES` | number
`IP` | string
`IS_FREE_TRIAL` | boolean
`IS_MASTER` | boolean
`LANDING_APP` | string
`PAPER_USER_NAME` | string
`QUALIFIED_FOR_MOBILE_AUTH` | boolean
`RESULT` | boolean
`SF_ENABLED` | boolean
`USER_ID` | number
`USER_NAME` | string
`features` | [SsoValidateResponseFeatures](SsoValidateResponseFeatures.md)
`lastAccessed` | number
`loginType` | number
`region` | string

## Example

```typescript
import type { SsoValidateResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "AUTH_TIME": null,
  "CREDENTIAL": null,
  "EXPIRES": null,
  "IP": null,
  "IS_FREE_TRIAL": null,
  "IS_MASTER": null,
  "LANDING_APP": null,
  "PAPER_USER_NAME": null,
  "QUALIFIED_FOR_MOBILE_AUTH": null,
  "RESULT": null,
  "SF_ENABLED": null,
  "USER_ID": null,
  "USER_NAME": null,
  "features": null,
  "lastAccessed": null,
  "loginType": null,
  "region": null,
} satisfies SsoValidateResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SsoValidateResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


