
# ResidenceAddress

Provide the residential address where the applicant physically resides. <br><ul><li>If the mailing address is different from the address provided in Residence element, THEN you will also include MailingAddress element.</li><li>Post Office Box is not accepted for Residential Address.</li><li>Our system validates street_1 and street_2 included within Residence attribute to ensure Post Office Box address is not provided.</li><li>An error will be thrown if the below combinations are included within street_1 OR street_2:</li><ul><li>PB</li><li>PO Box</li><li>Post Office Box</li><li>P.O. Box</li><li>In care of</li><li>General Delivery</li><li>Regular Expression to validate street_1 and street_2:</li></ul></ul>English: (?:P(?:ost(?:al)?)?[\\.\\-\\s]*(?:(?:O(?:ffice)?[\\.\\s]*)?B(?:ox|in|\\b|\\d)|o(?:ffice|\\b)(?:[-\\s]*\\d)|code)|box[-\\s]*\\d)<br>Chinese Simplified: PO Box    (?i)\\b((邮政信箱) [0-9]*)\\bChinese Traditional: PO Box   (?i)\\b((郵政信箱) [0-9]*)\\b

## Properties

Name | Type
------------ | -------------
`city` | string
`country` | string
`postalCode` | string
`state` | string
`street1` | string
`street2` | string

## Example

```typescript
import type { ResidenceAddress } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
  "city": London,
  "country": GBR,
  "postalCode": SW10 9QL,
  "state": GB-ENG,
  "street1": 1 Tester Street,
  "street2": null,
} satisfies ResidenceAddress

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ResidenceAddress
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


