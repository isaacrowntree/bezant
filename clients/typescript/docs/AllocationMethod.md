
# AllocationMethod

Interactive Brokers supports two forms of allocation methods. Allocation methods that have calculations completed by Interactive Brokers, and a set of allocation methods calculated by the user and then specified. IB-computed allocation methods:   * `A` - Available Equity   * `E` - Equal   * `N` - Net Liquidation Value  User-specified allocation methods:   * `C` - Cash Quantity   * `P` - Percentage   * `R` - Ratios   * `S` - Shares 

## Properties

Name | Type
------------ | -------------

## Example

```typescript
import type { AllocationMethod } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
} satisfies AllocationMethod

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AllocationMethod
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


