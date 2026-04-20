# AccountManagementReportsApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**getGwApiV1StatementsAvailable**](AccountManagementReportsApi.md#getgwapiv1statementsavailable) | **GET** /gw/api/v1/statements/available | Fetch Available Daily, Monthly, And Annual Report Dates For An Account Id |
| [**getGwApiV1TaxDocumentsAvailable**](AccountManagementReportsApi.md#getgwapiv1taxdocumentsavailable) | **GET** /gw/api/v1/tax-documents/available | Fetch List Of Available Tax Reports/forms/documents For A Specified Account And Tax Year |
| [**getGwApiV1TradeConfirmationsAvailable**](AccountManagementReportsApi.md#getgwapiv1tradeconfirmationsavailable) | **GET** /gw/api/v1/trade-confirmations/available | Fetch List Of Available Trade Confirmation Dates, For A Specific Account Id |
| [**postGwApiV1Statements**](AccountManagementReportsApi.md#postgwapiv1statements) | **POST** /gw/api/v1/statements | Generates Statements In Supported Formats Based On Request Parameters. |
| [**postGwApiV1TaxDocuments**](AccountManagementReportsApi.md#postgwapiv1taxdocuments) | **POST** /gw/api/v1/tax-documents | Fetch Tax Forms In Supported Formats Based On Request Parameters. |
| [**postGwApiV1TradeConfirmations**](AccountManagementReportsApi.md#postgwapiv1tradeconfirmations) | **POST** /gw/api/v1/trade-confirmations | Fetch Trade Confirmations In Supported Formats Based On Request Parameters. |



## getGwApiV1StatementsAvailable

> GetAvailableStmtDatesResponse getGwApiV1StatementsAvailable(authorization, accountId)

Fetch Available Daily, Monthly, And Annual Report Dates For An Account Id

&lt;br&gt;**Scope**: &#x60;statements.read&#x60; OR &#x60;reports.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementReportsApi,
} from 'bezant-client';
import type { GetGwApiV1StatementsAvailableRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementReportsApi();

  const body = {
    // string | Specifies the authorization header value (e.g., Bearer eyJ0eXAiOiJKV1...).
    authorization: Bearer eyJ0eXAiOiJKV1...,
    // string | Specifies the account id to retrieve information
    accountId: UXXXX,
  } satisfies GetGwApiV1StatementsAvailableRequest;

  try {
    const data = await api.getGwApiV1StatementsAvailable(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **authorization** | `string` | Specifies the authorization header value (e.g., Bearer eyJ0eXAiOiJKV1...). | [Defaults to `undefined`] |
| **accountId** | `string` | Specifies the account id to retrieve information | [Defaults to `undefined`] |

### Return type

[**GetAvailableStmtDatesResponse**](GetAvailableStmtDatesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns a JSON object containing the available report dates. |  -  |
| **400** | Returns a Problem detail instance representing a bad request. |  -  |
| **401** | Returns a Problem detail instance representing an unauthorized request. |  -  |
| **402** | Returns a Problem detail instance representing an unauthorized request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getGwApiV1TaxDocumentsAvailable

> GetAvailableTaxFormsResponse getGwApiV1TaxDocumentsAvailable(authorization, accountId, year)

Fetch List Of Available Tax Reports/forms/documents For A Specified Account And Tax Year

&lt;br&gt;**Scope**: &#x60;statements.read&#x60; OR &#x60;reports.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementReportsApi,
} from 'bezant-client';
import type { GetGwApiV1TaxDocumentsAvailableRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementReportsApi();

  const body = {
    // string | Specifies the authorization header value (e.g., Bearer eyJ0eXAiOiJKV1...).
    authorization: Bearer eyJ0eXAiOiJKV1...,
    // string | Specifies the account id to retrieve information
    accountId: UXXXX,
    // number | Specifies the tax year to retrieve information
    year: 2024,
  } satisfies GetGwApiV1TaxDocumentsAvailableRequest;

  try {
    const data = await api.getGwApiV1TaxDocumentsAvailable(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **authorization** | `string` | Specifies the authorization header value (e.g., Bearer eyJ0eXAiOiJKV1...). | [Defaults to `undefined`] |
| **accountId** | `string` | Specifies the account id to retrieve information | [Defaults to `undefined`] |
| **year** | `number` | Specifies the tax year to retrieve information | [Defaults to `undefined`] |

### Return type

[**GetAvailableTaxFormsResponse**](GetAvailableTaxFormsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns a JSON object containing the available report dates. |  -  |
| **400** | Returns a Problem detail instance representing a bad request. |  -  |
| **401** | Returns a Problem detail instance representing an unauthorized request. |  -  |
| **402** | Returns a Problem detail instance representing an unauthorized request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getGwApiV1TradeConfirmationsAvailable

> GetAvailableTradeConfirmationDatesResponse getGwApiV1TradeConfirmationsAvailable(authorization, accountId)

Fetch List Of Available Trade Confirmation Dates, For A Specific Account Id

&lt;br&gt;**Scope**: &#x60;statements.read&#x60; OR &#x60;reports.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementReportsApi,
} from 'bezant-client';
import type { GetGwApiV1TradeConfirmationsAvailableRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementReportsApi();

  const body = {
    // string | Specifies the authorization header value (e.g., Bearer eyJ0eXAiOiJKV1...).
    authorization: Bearer eyJ0eXAiOiJKV1...,
    // string | Specifies the account id to retrieve information
    accountId: UXXXX,
  } satisfies GetGwApiV1TradeConfirmationsAvailableRequest;

  try {
    const data = await api.getGwApiV1TradeConfirmationsAvailable(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **authorization** | `string` | Specifies the authorization header value (e.g., Bearer eyJ0eXAiOiJKV1...). | [Defaults to `undefined`] |
| **accountId** | `string` | Specifies the account id to retrieve information | [Defaults to `undefined`] |

### Return type

[**GetAvailableTradeConfirmationDatesResponse**](GetAvailableTradeConfirmationDatesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns a JSON object containing the available user-traded dates. |  -  |
| **400** | Returns a Problem detail instance representing a bad request. |  -  |
| **401** | Returns a Problem detail instance representing an unauthorized request. |  -  |
| **402** | Returns a Problem detail instance representing an unauthorized request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1Statements

> GetStatementsResponse postGwApiV1Statements(authorization, stmtRequest)

Generates Statements In Supported Formats Based On Request Parameters.

&lt;br&gt;**Scope**: &#x60;statements.read&#x60; OR &#x60;statements.write&#x60; OR &#x60;reports.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementReportsApi,
} from 'bezant-client';
import type { PostGwApiV1StatementsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementReportsApi();

  const body = {
    // string | Specifies the authorization header value (e.g., Bearer eyJ0eXAiOiJKV1...).
    authorization: Bearer eyJ0eXAiOiJKV1...,
    // StmtRequest | Report request object
    stmtRequest: ...,
  } satisfies PostGwApiV1StatementsRequest;

  try {
    const data = await api.postGwApiV1Statements(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **authorization** | `string` | Specifies the authorization header value (e.g., Bearer eyJ0eXAiOiJKV1...). | [Defaults to `undefined`] |
| **stmtRequest** | [StmtRequest](StmtRequest.md) | Report request object | |

### Return type

[**GetStatementsResponse**](GetStatementsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns a JSON object containing the relevant statement. |  -  |
| **400** | Returns a Problem detail instance representing a bad request. |  -  |
| **401** | Returns a Problem detail instance representing an unauthorized request. |  -  |
| **402** | Returns a Problem detail instance representing an unauthorized request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1TaxDocuments

> TaxFormResponse postGwApiV1TaxDocuments(authorization, taxFormRequest)

Fetch Tax Forms In Supported Formats Based On Request Parameters.

&lt;br&gt;**Scope**: &#x60;statements.write&#x60; OR &#x60;reports.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementReportsApi,
} from 'bezant-client';
import type { PostGwApiV1TaxDocumentsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementReportsApi();

  const body = {
    // string | Specifies the authorization header value (e.g., Bearer eyJ0eXAiOiJKV1...).
    authorization: Bearer eyJ0eXAiOiJKV1...,
    // TaxFormRequest | Tax Form request object
    taxFormRequest: ...,
  } satisfies PostGwApiV1TaxDocumentsRequest;

  try {
    const data = await api.postGwApiV1TaxDocuments(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **authorization** | `string` | Specifies the authorization header value (e.g., Bearer eyJ0eXAiOiJKV1...). | [Defaults to `undefined`] |
| **taxFormRequest** | [TaxFormRequest](TaxFormRequest.md) | Tax Form request object | |

### Return type

[**TaxFormResponse**](TaxFormResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns a JSON object containing the relevant Tax Form. |  -  |
| **400** | Returns a Problem detail instance representing a bad request. |  -  |
| **401** | Returns a Problem detail instance representing an unauthorized request. |  -  |
| **402** | Returns a Problem detail instance representing an unauthorized request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1TradeConfirmations

> TradeConfirmationResponse postGwApiV1TradeConfirmations(authorization, tradeConfirmationRequest)

Fetch Trade Confirmations In Supported Formats Based On Request Parameters.

&lt;br&gt;**Scope**: &#x60;statements.write&#x60; OR &#x60;reports.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementReportsApi,
} from 'bezant-client';
import type { PostGwApiV1TradeConfirmationsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementReportsApi();

  const body = {
    // string | Specifies the authorization header value (e.g., Bearer eyJ0eXAiOiJKV1...).
    authorization: Bearer eyJ0eXAiOiJKV1...,
    // TradeConfirmationRequest | Trade confirmation request body
    tradeConfirmationRequest: ...,
  } satisfies PostGwApiV1TradeConfirmationsRequest;

  try {
    const data = await api.postGwApiV1TradeConfirmations(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **authorization** | `string` | Specifies the authorization header value (e.g., Bearer eyJ0eXAiOiJKV1...). | [Defaults to `undefined`] |
| **tradeConfirmationRequest** | [TradeConfirmationRequest](TradeConfirmationRequest.md) | Trade confirmation request body | |

### Return type

[**TradeConfirmationResponse**](TradeConfirmationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns a JSON object containing the relevant Trade Confirmation. |  -  |
| **400** | Returns a Problem detail instance representing a bad request. |  -  |
| **401** | Returns a Problem detail instance representing an unauthorized request. |  -  |
| **402** | Returns a Problem detail instance representing an unauthorized request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

