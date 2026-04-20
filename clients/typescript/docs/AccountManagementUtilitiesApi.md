# AccountManagementUtilitiesApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**getGwApiV1EnumerationsComplexAssetTransfer**](AccountManagementUtilitiesApi.md#getgwapiv1enumerationscomplexassettransfer) | **GET** /gw/api/v1/enumerations/complex-asset-transfer | Get A List Of Participating Brokers For The Given Asset Type |
| [**getGwApiV1EnumerationsEnumerationtype**](AccountManagementUtilitiesApi.md#getgwapiv1enumerationsenumerationtype) | **GET** /gw/api/v1/enumerations/{enumerationType} | Get Enumerations |
| [**getGwApiV1Forms**](AccountManagementUtilitiesApi.md#getgwapiv1forms) | **GET** /gw/api/v1/forms | Get Forms |
| [**getGwApiV1FormsRequiredForms**](AccountManagementUtilitiesApi.md#getgwapiv1formsrequiredforms) | **GET** /gw/api/v1/forms/required-forms | Get Required Forms |
| [**getGwApiV1ParticipatingBanks**](AccountManagementUtilitiesApi.md#getgwapiv1participatingbanks) | **GET** /gw/api/v1/participating-banks | Get Participating Banks |
| [**getGwApiV1Requests**](AccountManagementUtilitiesApi.md#getgwapiv1requests) | **GET** /gw/api/v1/requests | Get Requests\&#39; Details By Timeframe |
| [**getGwApiV1ValidationsUsernamesUsername**](AccountManagementUtilitiesApi.md#getgwapiv1validationsusernamesusername) | **GET** /gw/api/v1/validations/usernames/{username} | Verify User Availability |
| [**patchGwApiV1RequestsRequestidStatus**](AccountManagementUtilitiesApi.md#patchgwapiv1requestsrequestidstatus) | **PATCH** /gw/api/v1/requests/{requestId}/status | Update Status Of An Am Request |
| [**postGwApiV1BalancesQuery**](AccountManagementUtilitiesApi.md#postgwapiv1balancesqueryoperation) | **POST** /gw/api/v1/balances/query | View Cash Balances |



## getGwApiV1EnumerationsComplexAssetTransfer

> GetBrokerListResponse getGwApiV1EnumerationsComplexAssetTransfer(instructionType, clientId)

Get A List Of Participating Brokers For The Given Asset Type

Get list of brokers supported for given asset transfer type&lt;br&gt;&lt;br&gt;**Scope**: &#x60;enumerations.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementUtilitiesApi,
} from 'bezant-client';
import type { GetGwApiV1EnumerationsComplexAssetTransferRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementUtilitiesApi();

  const body = {
    // string | Asset transfer type to get the list of supported brokers
    instructionType: COMPLEX_ASSET_TRANSFER,
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies GetGwApiV1EnumerationsComplexAssetTransferRequest;

  try {
    const data = await api.getGwApiV1EnumerationsComplexAssetTransfer(body);
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
| **instructionType** | `string` | Asset transfer type to get the list of supported brokers | [Defaults to `undefined`] |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**GetBrokerListResponse**](GetBrokerListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns list of brokers supported for given asset type |  -  |
| **400** | Returns a Problem detail instance representing a not found request. |  -  |
| **500** | Unable to process request due to an Internal Error. Please try again later. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getGwApiV1EnumerationsEnumerationtype

> EnumerationResponse getGwApiV1EnumerationsEnumerationtype(enumerationType, type, currency, ibEntity, mdStatusNonPro, formNumber, language, accountId, userName)

Get Enumerations

Used to query list of enumerations for attributes included within extPositionsTransfers, occupation, employerBusiness, financialInformation, affiliationDetails, tradingPermissions, etc.&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.read&#x60; OR &#x60;enumerations.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementUtilitiesApi,
} from 'bezant-client';
import type { GetGwApiV1EnumerationsEnumerationtypeRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementUtilitiesApi();

  const body = {
    // string
    enumerationType: enumerationType_example,
    // EnumerationType (optional)
    type: ...,
    // string (optional)
    currency: currency_example,
    // string (optional)
    ibEntity: ibEntity_example,
    // string (optional)
    mdStatusNonPro: mdStatusNonPro_example,
    // string (optional)
    formNumber: formNumber_example,
    // 'en' | 'ar' | 'de' | 'es' | 'fr' | 'he' | 'hu' | 'it' | 'ja' | 'nl' | 'pt' | 'ru' | 'zh_CN' | 'zh_TW' (optional)
    language: language_example,
    // string (optional)
    accountId: accountId_example,
    // string (optional)
    userName: userName_example,
  } satisfies GetGwApiV1EnumerationsEnumerationtypeRequest;

  try {
    const data = await api.getGwApiV1EnumerationsEnumerationtype(body);
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
| **enumerationType** | `string` |  | [Defaults to `undefined`] |
| **type** | `EnumerationType` |  | [Optional] [Defaults to `undefined`] [Enum: exchange-bundles, business-and-occupation, employee-track, fin-info-ranges, acats, aton, market-data, edd-avt, prohibited-country, employee-plans, questionnaires, security-questions, quiz-questions, wire-instructions, product-country-bundles] |
| **currency** | `string` |  | [Optional] [Defaults to `undefined`] |
| **ibEntity** | `string` |  | [Optional] [Defaults to `undefined`] |
| **mdStatusNonPro** | `string` |  | [Optional] [Defaults to `undefined`] |
| **formNumber** | `string` |  | [Optional] [Defaults to `undefined`] |
| **language** | `en`, `ar`, `de`, `es`, `fr`, `he`, `hu`, `it`, `ja`, `nl`, `pt`, `ru`, `zh_CN`, `zh_TW` |  | [Optional] [Defaults to `undefined`] [Enum: en, ar, de, es, fr, he, hu, it, ja, nl, pt, ru, zh_CN, zh_TW] |
| **accountId** | `string` |  | [Optional] [Defaults to `undefined`] |
| **userName** | `string` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**EnumerationResponse**](EnumerationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Get enumerations |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getGwApiV1Forms

> FormFileResponse getGwApiV1Forms(formNo, getDocs, fromDate, toDate, language, projection)

Get Forms

Get forms&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.read&#x60; OR &#x60;forms.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementUtilitiesApi,
} from 'bezant-client';
import type { GetGwApiV1FormsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementUtilitiesApi();

  const body = {
    // Array<string> (optional)
    formNo: ...,
    // string (optional)
    getDocs: getDocs_example,
    // string (optional)
    fromDate: fromDate_example,
    // string (optional)
    toDate: toDate_example,
    // string (optional)
    language: language_example,
    // 'PAYLOAD' | 'DOCS' | 'NONE' (optional)
    projection: projection_example,
  } satisfies GetGwApiV1FormsRequest;

  try {
    const data = await api.getGwApiV1Forms(body);
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
| **formNo** | `Array<string>` |  | [Optional] |
| **getDocs** | `string` |  | [Optional] [Defaults to `undefined`] |
| **fromDate** | `string` |  | [Optional] [Defaults to `undefined`] |
| **toDate** | `string` |  | [Optional] [Defaults to `undefined`] |
| **language** | `string` |  | [Optional] [Defaults to `undefined`] |
| **projection** | `PAYLOAD`, `DOCS`, `NONE` |  | [Optional] [Defaults to `&#39;NONE&#39;`] [Enum: PAYLOAD, DOCS, NONE] |

### Return type

[**FormFileResponse**](FormFileResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Get forms |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getGwApiV1FormsRequiredForms

> RequiredFormsResponse getGwApiV1FormsRequiredForms(requiredFormsRequest)

Get Required Forms

Get required forms&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementUtilitiesApi,
} from 'bezant-client';
import type { GetGwApiV1FormsRequiredFormsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementUtilitiesApi();

  const body = {
    // RequiredFormsRequest
    requiredFormsRequest: ...,
  } satisfies GetGwApiV1FormsRequiredFormsRequest;

  try {
    const data = await api.getGwApiV1FormsRequiredForms(body);
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
| **requiredFormsRequest** | [](.md) |  | [Defaults to `undefined`] |

### Return type

[**RequiredFormsResponse**](RequiredFormsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Get required forms |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getGwApiV1ParticipatingBanks

> GetParticipatingListResponse getGwApiV1ParticipatingBanks(type, clientId)

Get Participating Banks

Get list of banks which support banking connection with Interactive Brokers.&lt;br&gt;&lt;br&gt;**Scope**: &#x60;enumerations.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementUtilitiesApi,
} from 'bezant-client';
import type { GetGwApiV1ParticipatingBanksRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementUtilitiesApi();

  const body = {
    // string | Parameter for which the list of participating banks is fetched
    type: eDDA,
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies GetGwApiV1ParticipatingBanksRequest;

  try {
    const data = await api.getGwApiV1ParticipatingBanks(body);
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
| **type** | `string` | Parameter for which the list of participating banks is fetched | [Defaults to `undefined`] |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**GetParticipatingListResponse**](GetParticipatingListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns the list of participating banks. |  -  |
| **400** | Returns a Problem detail instance representing a not found request. |  -  |
| **500** | Unable to process request due to an Internal Error. Please try again later. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getGwApiV1Requests

> RequestDetailsResponse getGwApiV1Requests(requestDetails)

Get Requests\&#39; Details By Timeframe

Fetch Requests\&#39; Details By Timeframe&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementUtilitiesApi,
} from 'bezant-client';
import type { GetGwApiV1RequestsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementUtilitiesApi();

  const body = {
    // RequestDetailsRequest
    requestDetails: ...,
  } satisfies GetGwApiV1RequestsRequest;

  try {
    const data = await api.getGwApiV1Requests(body);
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
| **requestDetails** | [](.md) |  | [Defaults to `undefined`] |

### Return type

[**RequestDetailsResponse**](RequestDetailsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Fetch Requests\&#39; Details By Timeframe |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getGwApiV1ValidationsUsernamesUsername

> UserNameAvailableResponse getGwApiV1ValidationsUsernamesUsername(username)

Verify User Availability

Verify whether user is valid and available&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.read&#x60; OR &#x60;validations.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementUtilitiesApi,
} from 'bezant-client';
import type { GetGwApiV1ValidationsUsernamesUsernameRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementUtilitiesApi();

  const body = {
    // string
    username: username_example,
  } satisfies GetGwApiV1ValidationsUsernamesUsernameRequest;

  try {
    const data = await api.getGwApiV1ValidationsUsernamesUsername(body);
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
| **username** | `string` |  | [Defaults to `undefined`] |

### Return type

[**UserNameAvailableResponse**](UserNameAvailableResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Verify whether user is valid and available |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## patchGwApiV1RequestsRequestidStatus

> AmRequestStatusResponse patchGwApiV1RequestsRequestidStatus(requestId, body)

Update Status Of An Am Request

This api will be used to update the status of am request&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.read&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementUtilitiesApi,
} from 'bezant-client';
import type { PatchGwApiV1RequestsRequestidStatusRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementUtilitiesApi();

  const body = {
    // number
    requestId: 56,
    // string
    body: body_example,
  } satisfies PatchGwApiV1RequestsRequestidStatusRequest;

  try {
    const data = await api.patchGwApiV1RequestsRequestidStatus(body);
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
| **requestId** | `number` |  | [Defaults to `undefined`] |
| **body** | `string` |  | |

### Return type

[**AmRequestStatusResponse**](AmRequestStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `text/plain`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | This api will be used to update the status of am request |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1BalancesQuery

> QueryWithdrawableCashEquityResponse postGwApiV1BalancesQuery(postGwApiV1BalancesQueryRequest, clientId)

View Cash Balances

View available cash for withdrawal and account equity value by accountId&lt;br&gt;&lt;br&gt;**Scope**: &#x60;balances.read&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementUtilitiesApi,
} from 'bezant-client';
import type { PostGwApiV1BalancesQueryOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementUtilitiesApi();

  const body = {
    // PostGwApiV1BalancesQueryRequest | Create an query request body
    postGwApiV1BalancesQueryRequest: {"instruction":{"accountId":"U87440","clientInstructionId":7009005,"currency":"USD"},"instructionType":"QUERY_WITHDRAWABLE_CASH_EQUITY"},
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies PostGwApiV1BalancesQueryOperationRequest;

  try {
    const data = await api.postGwApiV1BalancesQuery(body);
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
| **postGwApiV1BalancesQueryRequest** | [PostGwApiV1BalancesQueryRequest](PostGwApiV1BalancesQueryRequest.md) | Create an query request body | |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**QueryWithdrawableCashEquityResponse**](QueryWithdrawableCashEquityResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Instruction successfully created and processed synchronously |  -  |
| **400** | Returns a Problem detail instance representing a bad request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **422** | Returns a Problem detail instance representing a business error. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

