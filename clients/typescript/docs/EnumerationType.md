
# EnumerationType

<ul><li>exchange-bundles - query most up to date list of exchange-bundles for tradingPermissions</li><li>business-and-occupation - list of occupation and employerBusiness for employmentDetails</li><li>employee-track - query most up to date companyId for account. For affiliation details, if company has an existing IBKR Employee Track account</li><li>fin-info-ranges - query most up to date range IDs by currency for annualNetIncome, netWorth, liquidNetWorth</li><li>acats - query most up to date values for brokerId and brokerName. Used if funding via US ACATS extPositionsTransfers</li><li>aton - query most up to date values for brokerId and brokerName. Used if funding via US ACATS extPositionsTransfers</li><li>market-data - query most up to date values for brokerId and brokerName. Used if funding via ATON Canada extPositionsTransfers</li><li>edd-avt - query questions associated with EDD (Enhanced Due Diligence) or AVT (Additional Verification) tasks assigned to an account</li><li>prohibited-country - view list of prohibited countries. Applicants that reside in prohibited country are restricted from opening an account with IBKR. Error will be thrown IF legalResidenceCountry, OR country (included within Residence, mailingAddress and employerAddress, taxResidency node) is a prohibited country</li><li>employee-plans - view EPA that are linked to master account (applicable IF offering SEP IRA accounts)</li><li>questionnaires - obtain list of questionnaires</li><li>security-questions - obtain list of questions supported for IBKR security questions</li><li>quiz-questions - obtain list of questions associated with IBKR knowledge assessment</li><li>wire-instructions - obtain list of wire instructions</li><li>product-country-bundles - obtain list of product country bundles</li></ul>

## Properties

Name | Type
------------ | -------------

## Example

```typescript
import type { EnumerationType } from 'bezant-client'

// TODO: Update the object below with actual values
const example = {
} satisfies EnumerationType

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as EnumerationType
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


