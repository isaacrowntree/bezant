# AccountManagementAccountsApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**getGwApiV1Accounts**](AccountManagementAccountsApi.md#getgwapiv1accounts) | **GET** /gw/api/v1/accounts | Retrieve Processed Application |
| [**getGwApiV1AccountsAccountidDetails**](AccountManagementAccountsApi.md#getgwapiv1accountsaccountiddetails) | **GET** /gw/api/v1/accounts/{accountId}/details | Get Account Information |
| [**getGwApiV1AccountsAccountidKyc**](AccountManagementAccountsApi.md#getgwapiv1accountsaccountidkyc) | **GET** /gw/api/v1/accounts/{accountId}/kyc | Retrieve Au10Tix URL |
| [**getGwApiV1AccountsAccountidLoginMessages**](AccountManagementAccountsApi.md#getgwapiv1accountsaccountidloginmessages) | **GET** /gw/api/v1/accounts/{accountId}/login-messages | Get Login Message By Account |
| [**getGwApiV1AccountsAccountidStatus**](AccountManagementAccountsApi.md#getgwapiv1accountsaccountidstatus) | **GET** /gw/api/v1/accounts/{accountId}/status | Get Status By Account |
| [**getGwApiV1AccountsAccountidTasks**](AccountManagementAccountsApi.md#getgwapiv1accountsaccountidtasks) | **GET** /gw/api/v1/accounts/{accountId}/tasks | Get Registration Tasks |
| [**getGwApiV1AccountsLoginMessages**](AccountManagementAccountsApi.md#getgwapiv1accountsloginmessages) | **GET** /gw/api/v1/accounts/login-messages | Get Login Messages |
| [**getGwApiV1AccountsStatus**](AccountManagementAccountsApi.md#getgwapiv1accountsstatus) | **GET** /gw/api/v1/accounts/status | Get Status Of Accounts |
| [**patchGwApiV1Accounts**](AccountManagementAccountsApi.md#patchgwapiv1accounts) | **PATCH** /gw/api/v1/accounts | Update Account |
| [**patchGwApiV1AccountsAccountidStatus**](AccountManagementAccountsApi.md#patchgwapiv1accountsaccountidstatus) | **PATCH** /gw/api/v1/accounts/{accountId}/status | Update Status By Account |
| [**patchGwApiV1AccountsAccountidTasks**](AccountManagementAccountsApi.md#patchgwapiv1accountsaccountidtasks) | **PATCH** /gw/api/v1/accounts/{accountId}/tasks | Update Tasks Status |
| [**postGwApiV1Accounts**](AccountManagementAccountsApi.md#postgwapiv1accounts) | **POST** /gw/api/v1/accounts | Create Account |
| [**postGwApiV1AccountsAccountidTasks**](AccountManagementAccountsApi.md#postgwapiv1accountsaccountidtasks) | **POST** /gw/api/v1/accounts/{accountId}/tasks | Assign Tasks |
| [**postGwApiV1AccountsDocuments**](AccountManagementAccountsApi.md#postgwapiv1accountsdocuments) | **POST** /gw/api/v1/accounts/documents | Submit General Agreements And Disclosures |



## getGwApiV1Accounts

> GetGwApiV1Accounts200Response getGwApiV1Accounts(accountId, externalId)

Retrieve Processed Application

Retrieve the application request and IBKR response data based on IBKR accountId or externalId. Only available for accounts that originate via API&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementAccountsApi,
} from 'bezant-client';
import type { GetGwApiV1AccountsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementAccountsApi();

  const body = {
    // string (optional)
    accountId: accountId_example,
    // string (optional)
    externalId: externalId_example,
  } satisfies GetGwApiV1AccountsRequest;

  try {
    const data = await api.getGwApiV1Accounts(body);
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
| **accountId** | `string` |  | [Optional] [Defaults to `undefined`] |
| **externalId** | `string` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**GetGwApiV1Accounts200Response**](GetGwApiV1Accounts200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Retrieve the application request and IBKR response data based on IBKR accountId or externalId. Only available for accounts that originate via API |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getGwApiV1AccountsAccountidDetails

> AccountDetailsResponse getGwApiV1AccountsAccountidDetails(accountId)

Get Account Information

&lt;br&gt;**Scope**: &#x60;accounts.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementAccountsApi,
} from 'bezant-client';
import type { GetGwApiV1AccountsAccountidDetailsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementAccountsApi();

  const body = {
    // string
    accountId: accountId_example,
  } satisfies GetGwApiV1AccountsAccountidDetailsRequest;

  try {
    const data = await api.getGwApiV1AccountsAccountidDetails(body);
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
| **accountId** | `string` |  | [Defaults to `undefined`] |

### Return type

[**AccountDetailsResponse**](AccountDetailsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | View information associated with account including contact data, financial information and trading configuration. |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getGwApiV1AccountsAccountidKyc

> Au10TixDetailResponse getGwApiV1AccountsAccountidKyc(accountId)

Retrieve Au10Tix URL

Generate URL address to complete real-time KYC verification using Au10Tix&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementAccountsApi,
} from 'bezant-client';
import type { GetGwApiV1AccountsAccountidKycRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementAccountsApi();

  const body = {
    // string
    accountId: accountId_example,
  } satisfies GetGwApiV1AccountsAccountidKycRequest;

  try {
    const data = await api.getGwApiV1AccountsAccountidKyc(body);
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
| **accountId** | `string` |  | [Defaults to `undefined`] |

### Return type

[**Au10TixDetailResponse**](Au10TixDetailResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Generate URL address to complete real-time KYC verification using Au10Tix |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getGwApiV1AccountsAccountidLoginMessages

> LoginMessageResponse getGwApiV1AccountsAccountidLoginMessages(accountId, type)

Get Login Message By Account

Query login messages assigned by accountId&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementAccountsApi,
} from 'bezant-client';
import type { GetGwApiV1AccountsAccountidLoginMessagesRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementAccountsApi();

  const body = {
    // string
    accountId: accountId_example,
    // string (optional)
    type: type_example,
  } satisfies GetGwApiV1AccountsAccountidLoginMessagesRequest;

  try {
    const data = await api.getGwApiV1AccountsAccountidLoginMessages(body);
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
| **accountId** | `string` |  | [Defaults to `undefined`] |
| **type** | `string` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**LoginMessageResponse**](LoginMessageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Query login messages assigned by accountId |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getGwApiV1AccountsAccountidStatus

> AccountStatusResponse getGwApiV1AccountsAccountidStatus(accountId)

Get Status By Account

Query status of account by accountId&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementAccountsApi,
} from 'bezant-client';
import type { GetGwApiV1AccountsAccountidStatusRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementAccountsApi();

  const body = {
    // string
    accountId: accountId_example,
  } satisfies GetGwApiV1AccountsAccountidStatusRequest;

  try {
    const data = await api.getGwApiV1AccountsAccountidStatus(body);
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
| **accountId** | `string` |  | [Defaults to `undefined`] |

### Return type

[**AccountStatusResponse**](AccountStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Query status of account by accountId. |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getGwApiV1AccountsAccountidTasks

> GetGwApiV1AccountsAccountidTasks200Response getGwApiV1AccountsAccountidTasks(accountId, type)

Get Registration Tasks

Query registration tasks assigned to account and pending tasks that are required for account approval&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementAccountsApi,
} from 'bezant-client';
import type { GetGwApiV1AccountsAccountidTasksRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementAccountsApi();

  const body = {
    // string
    accountId: accountId_example,
    // 'pending' | 'registration' (optional)
    type: type_example,
  } satisfies GetGwApiV1AccountsAccountidTasksRequest;

  try {
    const data = await api.getGwApiV1AccountsAccountidTasks(body);
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
| **accountId** | `string` |  | [Defaults to `undefined`] |
| **type** | `pending`, `registration` |  | [Optional] [Defaults to `&#39;registration&#39;`] [Enum: pending, registration] |

### Return type

[**GetGwApiV1AccountsAccountidTasks200Response**](GetGwApiV1AccountsAccountidTasks200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Query registration tasks assigned to account and pending tasks that are required for account approval |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getGwApiV1AccountsLoginMessages

> LoginMessageResponse getGwApiV1AccountsLoginMessages(loginMessageRequest)

Get Login Messages

Query all accounts associated with ‘Client ID’ that have incomplete login message&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementAccountsApi,
} from 'bezant-client';
import type { GetGwApiV1AccountsLoginMessagesRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementAccountsApi();

  const body = {
    // LoginMessageRequest
    loginMessageRequest: ...,
  } satisfies GetGwApiV1AccountsLoginMessagesRequest;

  try {
    const data = await api.getGwApiV1AccountsLoginMessages(body);
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
| **loginMessageRequest** | [](.md) |  | [Defaults to `undefined`] |

### Return type

[**LoginMessageResponse**](LoginMessageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Query all accounts associated with ‘Client ID’ that have incomplete login message |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getGwApiV1AccountsStatus

> AccountStatusBulkResponse getGwApiV1AccountsStatus(accountStatusRequest)

Get Status Of Accounts

Query status of all accounts associated with ‘Client ID\&#39;&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementAccountsApi,
} from 'bezant-client';
import type { GetGwApiV1AccountsStatusRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementAccountsApi();

  const body = {
    // AccountStatusRequest
    accountStatusRequest: ...,
  } satisfies GetGwApiV1AccountsStatusRequest;

  try {
    const data = await api.getGwApiV1AccountsStatus(body);
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
| **accountStatusRequest** | [](.md) |  | [Defaults to `undefined`] |

### Return type

[**AccountStatusBulkResponse**](AccountStatusBulkResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Query status of all accounts associated with ‘Client ID\&#39; |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## patchGwApiV1Accounts

> StatusResponse patchGwApiV1Accounts(body)

Update Account

Update information for an existing accountId&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementAccountsApi,
} from 'bezant-client';
import type { PatchGwApiV1AccountsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementAccountsApi();

  const body = {
    // string
    body: body_example,
  } satisfies PatchGwApiV1AccountsRequest;

  try {
    const data = await api.patchGwApiV1Accounts(body);
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
| **body** | `string` |  | |

### Return type

[**StatusResponse**](StatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `text/plain`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Update information for an existing accountId |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## patchGwApiV1AccountsAccountidStatus

> AccountStatusResponse patchGwApiV1AccountsAccountidStatus(accountId, body)

Update Status By Account

Query status of account by accountId&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementAccountsApi,
} from 'bezant-client';
import type { PatchGwApiV1AccountsAccountidStatusRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementAccountsApi();

  const body = {
    // string
    accountId: accountId_example,
    // string
    body: body_example,
  } satisfies PatchGwApiV1AccountsAccountidStatusRequest;

  try {
    const data = await api.patchGwApiV1AccountsAccountidStatus(body);
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
| **accountId** | `string` |  | [Defaults to `undefined`] |
| **body** | `string` |  | |

### Return type

[**AccountStatusResponse**](AccountStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `text/plain`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Query status of account by accountId. |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## patchGwApiV1AccountsAccountidTasks

> Array&lt;TaskActionResponse&gt; patchGwApiV1AccountsAccountidTasks(accountId, body)

Update Tasks Status

Task status can be updated from this api&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementAccountsApi,
} from 'bezant-client';
import type { PatchGwApiV1AccountsAccountidTasksRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementAccountsApi();

  const body = {
    // string
    accountId: accountId_example,
    // string
    body: body_example,
  } satisfies PatchGwApiV1AccountsAccountidTasksRequest;

  try {
    const data = await api.patchGwApiV1AccountsAccountidTasks(body);
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
| **accountId** | `string` |  | [Defaults to `undefined`] |
| **body** | `string` |  | |

### Return type

[**Array&lt;TaskActionResponse&gt;**](TaskActionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `text/plain`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Task status can be updated from this api |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1Accounts

> StatusResponse postGwApiV1Accounts(body)

Create Account

Submit account application. This will create brokerage account for the end user.&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementAccountsApi,
} from 'bezant-client';
import type { PostGwApiV1AccountsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementAccountsApi();

  const body = {
    // string
    body: body_example,
  } satisfies PostGwApiV1AccountsRequest;

  try {
    const data = await api.postGwApiV1Accounts(body);
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
| **body** | `string` |  | |

### Return type

[**StatusResponse**](StatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `text/plain`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Submit account application. This will create brokerage account for the end user. |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1AccountsAccountidTasks

> Array&lt;TaskActionResponse&gt; postGwApiV1AccountsAccountidTasks(accountId, body)

Assign Tasks

Task can be assigned from this api&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementAccountsApi,
} from 'bezant-client';
import type { PostGwApiV1AccountsAccountidTasksRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementAccountsApi();

  const body = {
    // string
    accountId: accountId_example,
    // string
    body: body_example,
  } satisfies PostGwApiV1AccountsAccountidTasksRequest;

  try {
    const data = await api.postGwApiV1AccountsAccountidTasks(body);
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
| **accountId** | `string` |  | [Defaults to `undefined`] |
| **body** | `string` |  | |

### Return type

[**Array&lt;TaskActionResponse&gt;**](TaskActionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `text/plain`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Task can be assigned from this api |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1AccountsDocuments

> StatusResponse postGwApiV1AccountsDocuments(body)

Submit General Agreements And Disclosures

Provides mechanism to submit Agreements and Disclosures to IBKR once a day instead of with each application. We store these documents on the servers and will use them for new application requests submitted that day.&lt;ul&gt;&lt;li&gt;Documents will need to be submitted once a day (before the Applications are submitted). PDFs will be displayed and submitted as is- no changes/edits will be made to the actual PDF files.&lt;/li&gt;&lt;li&gt;This end-point will not process any Tax Form Documents. Tax Form document should be submitted with every application&lt;/li&gt;&lt;li&gt;If submitted in the morning, you only need to include the Tax Form attachment for each applicant. Otherwise, you will need to include PDFs with each application (Create Account).&lt;/li&gt;&lt;/ul&gt;&lt;br&gt;&lt;br&gt;**Scope**: &#x60;accounts.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementAccountsApi,
} from 'bezant-client';
import type { PostGwApiV1AccountsDocumentsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementAccountsApi();

  const body = {
    // string
    body: body_example,
  } satisfies PostGwApiV1AccountsDocumentsRequest;

  try {
    const data = await api.postGwApiV1AccountsDocuments(body);
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
| **body** | `string` |  | |

### Return type

[**StatusResponse**](StatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `text/plain`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Submit standard agreements and disclosures |  -  |
| **400** | Returns error description representing bad request |  -  |
| **401** | Returns error description representing access issue |  -  |
| **403** | Returns error description representing access issue |  -  |
| **415** | Unsupported Media Type |  -  |
| **500** | Returns error description representing internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

