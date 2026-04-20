# TradingAccountsApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**getAccountMarketSummary**](TradingAccountsApi.md#getaccountmarketsummary) | **GET** /iserver/account/{accountId}/summary/market_value | Summary Of Account Market Value |
| [**getAccountOwners**](TradingAccountsApi.md#getaccountowners) | **GET** /acesws/{accountId}/signatures-and-owners | List Account Signatures And Owners |
| [**getAccountSummary**](TradingAccountsApi.md#getaccountsummary) | **GET** /iserver/account/{accountId}/summary | Summary Of Account Values |
| [**getBalanceSummary**](TradingAccountsApi.md#getbalancesummary) | **GET** /iserver/account/{accountId}/summary/balances | Summary Of Account Balances |
| [**getBrokerageAccounts**](TradingAccountsApi.md#getbrokerageaccounts) | **GET** /iserver/accounts | List All Tradable Accounts |
| [**getDynamicAccounts**](TradingAccountsApi.md#getdynamicaccounts) | **GET** /iserver/account/search/{searchPattern} | Search Dynamic Accounts |
| [**getFundSummary**](TradingAccountsApi.md#getfundsummary) | **GET** /iserver/account/{accountId}/summary/available_funds | Summary Of Available Funds |
| [**getMarginSummary**](TradingAccountsApi.md#getmarginsummary) | **GET** /iserver/account/{accountId}/summary/margins | Summary Of Account Margin Usage |
| [**getPnl**](TradingAccountsApi.md#getpnl) | **GET** /iserver/account/pnl/partitioned | Account Profit And Loss |
| [**setActiveAccount**](TradingAccountsApi.md#setactiveaccountoperation) | **POST** /iserver/account | Switch Selected Account |
| [**setDynamicAccount**](TradingAccountsApi.md#setdynamicaccountoperation) | **POST** /iserver/dynaccount | Set Active Dynamic Account |



## getAccountMarketSummary

> SummaryMarketValueResponse getAccountMarketSummary(accountId)

Summary Of Account Market Value

Returns a summary of an account\&#39;s market value, by currency and asset class.

### Example

```ts
import {
  Configuration,
  TradingAccountsApi,
} from 'bezant-client';
import type { GetAccountMarketSummaryRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingAccountsApi();

  const body = {
    // string
    accountId: U1234567,
  } satisfies GetAccountMarketSummaryRequest;

  try {
    const data = await api.getAccountMarketSummary(body);
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

[**SummaryMarketValueResponse**](SummaryMarketValueResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Indicates a successful market value request. |  -  |
| **400** | bad request; passed input cannot pass initial validation and detected right away |  -  |
| **401** | Invalid or expired access token |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getAccountOwners

> SignatureAndOwners getAccountOwners(accountId)

List Account Signatures And Owners

Receive a list of all applicant names on the account and for which account and entity is represented.

### Example

```ts
import {
  Configuration,
  TradingAccountsApi,
} from 'bezant-client';
import type { GetAccountOwnersRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingAccountsApi();

  const body = {
    // string
    accountId: accountId_example,
  } satisfies GetAccountOwnersRequest;

  try {
    const data = await api.getAccountOwners(body);
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

[**SignatureAndOwners**](SignatureAndOwners.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | An object containing valid accounts and the account properties regarding trading access. This endpoint is also used to confirm account validation. |  -  |
| **400** | bad request; accountId is empty |  -  |
| **401** | Invalid or expired access token |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getAccountSummary

> AccountSummaryResponse getAccountSummary(accountId)

Summary Of Account Values

Provides a general overview of the account details such as balance values.

### Example

```ts
import {
  Configuration,
  TradingAccountsApi,
} from 'bezant-client';
import type { GetAccountSummaryRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingAccountsApi();

  const body = {
    // string
    accountId: U1234567,
  } satisfies GetAccountSummaryRequest;

  try {
    const data = await api.getAccountSummary(body);
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

[**AccountSummaryResponse**](AccountSummaryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Provides a general overview of the account details such as balance values. |  -  |
| **400** | bad request; passed input cannot pass initial validation and detected right away |  -  |
| **401** | Invalid or expired access token |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getBalanceSummary

> SummaryOfAccountBalancesResponse getBalanceSummary(accountId)

Summary Of Account Balances

Returns a summary of an account\&#39;s equity and cash balances, in total and by account segment.

### Example

```ts
import {
  Configuration,
  TradingAccountsApi,
} from 'bezant-client';
import type { GetBalanceSummaryRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingAccountsApi();

  const body = {
    // string
    accountId: U1234567,
  } satisfies GetBalanceSummaryRequest;

  try {
    const data = await api.getBalanceSummary(body);
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

[**SummaryOfAccountBalancesResponse**](SummaryOfAccountBalancesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Indicates a successful return of available funds. |  -  |
| **400** | bad request; passed input cannot pass initial validation and detected right away |  -  |
| **401** | Invalid or expired access token |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getBrokerageAccounts

> UserAccountsResponse getBrokerageAccounts()

List All Tradable Accounts

Returns a list of accounts the user has trading access to, their respective aliases and the currently selected account. Note this endpoint must be called before modifying an order or querying open orders.

### Example

```ts
import {
  Configuration,
  TradingAccountsApi,
} from 'bezant-client';
import type { GetBrokerageAccountsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingAccountsApi();

  try {
    const data = await api.getBrokerageAccounts();
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**UserAccountsResponse**](UserAccountsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | An object containing valid accounts and the account properties regarding trading access. This endpoint is also used to confirm account validation. |  -  |
| **401** | Invalid or expired access token |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getDynamicAccounts

> DynAccountSearchResponse getDynamicAccounts(searchPattern)

Search Dynamic Accounts

Returns a list of accounts matching a query pattern set in the request. Broker accounts configured with the DYNACCT property will not receive account information at login. Instead, they must dynamically query then set their account number. Customers without the DYNACCT property will receive a 503 error. 

### Example

```ts
import {
  Configuration,
  TradingAccountsApi,
} from 'bezant-client';
import type { GetDynamicAccountsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingAccountsApi();

  const body = {
    // string
    searchPattern: U123,
  } satisfies GetDynamicAccountsRequest;

  try {
    const data = await api.getDynamicAccounts(body);
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
| **searchPattern** | `string` |  | [Defaults to `undefined`] |

### Return type

[**DynAccountSearchResponse**](DynAccountSearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns all accounts that match the searchPattern string. |  -  |
| **401** | Invalid or expired access token |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getFundSummary

> AvailableFundsResponse getFundSummary(accountId)

Summary Of Available Funds

Provides a summary specific for avilable funds giving more depth than the standard /summary endpoint.

### Example

```ts
import {
  Configuration,
  TradingAccountsApi,
} from 'bezant-client';
import type { GetFundSummaryRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingAccountsApi();

  const body = {
    // string
    accountId: U1234567,
  } satisfies GetFundSummaryRequest;

  try {
    const data = await api.getFundSummary(body);
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

[**AvailableFundsResponse**](AvailableFundsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Indicates a successful return of available funds. |  -  |
| **400** | bad request; passed input cannot pass initial validation and detected right away |  -  |
| **401** | Invalid or expired access token |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getMarginSummary

> SummaryOfAccountMarginResponse getMarginSummary(accountId)

Summary Of Account Margin Usage

Returns a summary of an account\&#39;s margin, in total and by account segment.

### Example

```ts
import {
  Configuration,
  TradingAccountsApi,
} from 'bezant-client';
import type { GetMarginSummaryRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingAccountsApi();

  const body = {
    // string
    accountId: U1234567,
  } satisfies GetMarginSummaryRequest;

  try {
    const data = await api.getMarginSummary(body);
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

[**SummaryOfAccountMarginResponse**](SummaryOfAccountMarginResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Indicates a success request to receive margin balance values. |  -  |
| **400** | bad request; passed input cannot pass initial validation and detected right away |  -  |
| **401** | Invalid or expired access token |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getPnl

> PnlPartitionedResponse getPnl()

Account Profit And Loss

Returns updated profit and loss values for the selected account. Initial request will return an empty array in the upnl object. 

### Example

```ts
import {
  Configuration,
  TradingAccountsApi,
} from 'bezant-client';
import type { GetPnlRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingAccountsApi();

  try {
    const data = await api.getPnl();
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**PnlPartitionedResponse**](PnlPartitionedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Refers to \&quot;updated PnL\&quot;. Holds a json object of key-value paired pnl details. |  -  |
| **400** | bad request; passed input cannot pass initial validation and detected right away |  -  |
| **401** | Invalid or expired access token |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## setActiveAccount

> SetAccountResponse setActiveAccount(setActiveAccountRequest)

Switch Selected Account

Switch the active account for how you request data. Only available for financial advisors and multi-account structures. 

### Example

```ts
import {
  Configuration,
  TradingAccountsApi,
} from 'bezant-client';
import type { SetActiveAccountOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingAccountsApi();

  const body = {
    // SetActiveAccountRequest
    setActiveAccountRequest: ...,
  } satisfies SetActiveAccountOperationRequest;

  try {
    const data = await api.setActiveAccount(body);
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
| **setActiveAccountRequest** | [SetActiveAccountRequest](SetActiveAccountRequest.md) |  | |

### Return type

[**SetAccountResponse**](SetAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Validates the account swapped to. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | Internal Server Error. Unable to process request if incoming values are not valid. For example accountId is not correct  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## setDynamicAccount

> SetAccountResponse setDynamicAccount(setDynamicAccountRequest)

Set Active Dynamic Account

Set the active dynamic account.

### Example

```ts
import {
  Configuration,
  TradingAccountsApi,
} from 'bezant-client';
import type { SetDynamicAccountOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingAccountsApi();

  const body = {
    // SetDynamicAccountRequest
    setDynamicAccountRequest: ...,
  } satisfies SetDynamicAccountOperationRequest;

  try {
    const data = await api.setDynamicAccount(body);
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
| **setDynamicAccountRequest** | [SetDynamicAccountRequest](SetDynamicAccountRequest.md) |  | |

### Return type

[**SetAccountResponse**](SetAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | An array of objects detailing contract information. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

