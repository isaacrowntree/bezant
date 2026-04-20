
# ScheduleResponse


## Properties

Name | Type
------------ | -------------
`exchange_time_zone` | string
`schedules` | [ScheduleResponseSchedules](ScheduleResponseSchedules.md)

## Example

```typescript
import type { ScheduleResponse } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "exchange_time_zone": null,
  "schedules": null,
} satisfies ScheduleResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ScheduleResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


