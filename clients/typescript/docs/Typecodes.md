
# Typecodes

Many FYI endpoints reference a \"typecode\" value. The table below lists the available codes and what they correspond to.   * `BA` - Borrow Availability   * `CA` - Comparable Algo   * `DA` - Dividends Advisory   * `EA` - Upcoming Earnings   * `MF` - Mutual Fund Advisory   * `OE` - Option Expiration   * `PR` - Portfolio Builder Rebalance   * `SE` - Suspend Order on Economic Event   * `SG` - Short Term Gain turning Long Term   * `SM` - System Messages   * `T2` - Assignment Realizing Long-Term Gains   * `TO` - Takeover   * `UA` - User Alert   * `M8` - M871 Trades   * `PS` - Platform Use Suggestions   * `DL` - Unexercised Option Loss Prevention Reminder   * `PT` - Position Transfer   * `CB` - Missing Cost Basis   * `MS` - Milestones   * `TD` - MiFID || 10% Deprecation Notice   * `ST` - Save Taxes   * `TI` - Trade Idea   * `CT` - Cash Transfer 

## Properties

Name | Type
------------ | -------------

## Example

```typescript
import type { Typecodes } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
} satisfies Typecodes

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Typecodes
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


