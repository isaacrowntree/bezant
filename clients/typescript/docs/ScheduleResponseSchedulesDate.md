
# ScheduleResponseSchedulesDate

object containing the  of hours objects detailing extended and liquid trading.

## Properties

Name | Type
------------ | -------------
`extended_hours` | [Array&lt;ScheduleResponseSchedulesDateExtendedHoursInner&gt;](ScheduleResponseSchedulesDateExtendedHoursInner.md)
`liquid_hours` | [Array&lt;ScheduleResponseSchedulesDateLiquidHoursInner&gt;](ScheduleResponseSchedulesDateLiquidHoursInner.md)

## Example

```typescript
import type { ScheduleResponseSchedulesDate } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "extended_hours": null,
  "liquid_hours": null,
} satisfies ScheduleResponseSchedulesDate

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ScheduleResponseSchedulesDate
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


