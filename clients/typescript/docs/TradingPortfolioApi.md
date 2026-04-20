# TradingPortfolioApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**getAllAccounts**](TradingPortfolioApi.md#getallaccounts) | **GET** /portfolio/accounts | List All Accounts |
| [**getAllAccountsForConid**](TradingPortfolioApi.md#getallaccountsforconid) | **GET** /portfolio/positions/{conid} | All Account Positions In An Instrument |
| [**getAllSubaccounts**](TradingPortfolioApi.md#getallsubaccounts) | **GET** /portfolio/subaccounts | List All Subaccounts |
| [**getAssetAllocation**](TradingPortfolioApi.md#getassetallocation) | **GET** /portfolio/{accountId}/allocation | Account Allocations |
| [**getComboPositions**](TradingPortfolioApi.md#getcombopositions) | **GET** /portfolio/{accountId}/combo/positions | Combination Positions |
| [**getManySubaccounts**](TradingPortfolioApi.md#getmanysubaccounts) | **GET** /portfolio/subaccounts2 | Portfolio Subaccounts (Large Account Structures) |
| [**getPaginatedPositions**](TradingPortfolioApi.md#getpaginatedpositions) | **GET** /portfolio/{accountId}/positions/{pageId} | Account Positions |
| [**getPortfolioLedger**](TradingPortfolioApi.md#getportfolioledger) | **GET** /portfolio/{accountId}/ledger | Account Ledger |
| [**getPortfolioMetadata**](TradingPortfolioApi.md#getportfoliometadata) | **GET** /portfolio/{accountId}/meta | Account Attributes |
| [**getPortfolioSummary**](TradingPortfolioApi.md#getportfoliosummary) | **GET** /portfolio/{accountId}/summary | Account Portfolio Summary |
| [**getPositionByConid**](TradingPortfolioApi.md#getpositionbyconid) | **GET** /portfolio/{accountid}/position/{conid} | Account Position In An Instrument |
| [**getUncachedPositions**](TradingPortfolioApi.md#getuncachedpositions) | **GET** /portfolio2/{accountId}/positions | Account Positions (NEW) |
| [**invalidatePositionCache**](TradingPortfolioApi.md#invalidatepositioncache) | **POST** /portfolio/{accountId}/positions/invalidate | Refresh Position Cache |



## getAllAccounts

> Array&lt;AccountAttributes&gt; getAllAccounts()

List All Accounts

return accounts

### Example

```ts
import {
  Configuration,
  TradingPortfolioApi,
} from 'bezant-client';
import type { GetAllAccountsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingPortfolioApi();

  try {
    const data = await api.getAllAccounts();
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

[**Array&lt;AccountAttributes&gt;**](AccountAttributes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | returned array with user account |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getAllAccountsForConid

> { [key: string]: IndividualPosition; } getAllAccountsForConid(conid)

All Account Positions In An Instrument

Get positions in accounts for a given instrument (no secDef await control)

### Example

```ts
import {
  Configuration,
  TradingPortfolioApi,
} from 'bezant-client';
import type { GetAllAccountsForConidRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingPortfolioApi();

  const body = {
    // number
    conid: 8314,
  } satisfies GetAllAccountsForConidRequest;

  try {
    const data = await api.getAllAccountsForConid(body);
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
| **conid** | `number` |  | [Defaults to `undefined`] |

### Return type

[**{ [key: string]: IndividualPosition; }**](IndividualPosition.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Object containing positions in the requested conid broken out by account. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getAllSubaccounts

> Array&lt;AccountAttributes&gt; getAllSubaccounts()

List All Subaccounts

Retrieve attributes of the subaccounts in the account structure.

### Example

```ts
import {
  Configuration,
  TradingPortfolioApi,
} from 'bezant-client';
import type { GetAllSubaccountsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingPortfolioApi();

  try {
    const data = await api.getAllSubaccounts();
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

[**Array&lt;AccountAttributes&gt;**](AccountAttributes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Array of objects representing accounts in the structure. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getAssetAllocation

> PortfolioAllocations getAssetAllocation(accountId, model)

Account Allocations

Get an account\&#39;s allocations by asset class, sector group, and sector.

### Example

```ts
import {
  Configuration,
  TradingPortfolioApi,
} from 'bezant-client';
import type { GetAssetAllocationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingPortfolioApi();

  const body = {
    // string
    accountId: accountId_example,
    // any (optional)
    model: ...,
  } satisfies GetAssetAllocationRequest;

  try {
    const data = await api.getAssetAllocation(body);
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
| **model** | `any` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**PortfolioAllocations**](PortfolioAllocations.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | response with allocations |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getComboPositions

> Array&lt;ComboPositionResponseInner&gt; getComboPositions(accountId, nocache)

Combination Positions

Provides all positions held in the account acquired as a combination, including values such as ratios, size, and market value.

### Example

```ts
import {
  Configuration,
  TradingPortfolioApi,
} from 'bezant-client';
import type { GetComboPositionsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingPortfolioApi();

  const body = {
    // string
    accountId: accountId_example,
    // boolean (optional)
    nocache: true,
  } satisfies GetComboPositionsRequest;

  try {
    const data = await api.getComboPositions(body);
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
| **nocache** | `boolean` |  | [Optional] [Defaults to `false`] |

### Return type

[**Array&lt;ComboPositionResponseInner&gt;**](ComboPositionResponseInner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | response with combo position definitions |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getManySubaccounts

> Subaccounts2Response getManySubaccounts(accountId, nocache)

Portfolio Subaccounts (Large Account Structures)

Used in tiered account structures (such as Financial Advisor and IBroker Accounts) to return a list of sub-accounts, paginated up to 20 accounts per page, for which the user can view position and account-related information.  This endpoint must be called prior to calling other /portfolio endpoints for those sub-accounts. If you have less than 100 sub-accounts use /portfolio/subaccounts.  To query a list of accounts the user can trade, see /iserver/accounts. 

### Example

```ts
import {
  Configuration,
  TradingPortfolioApi,
} from 'bezant-client';
import type { GetManySubaccountsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingPortfolioApi();

  const body = {
    // string (optional)
    accountId: DU123456,
    // boolean (optional)
    nocache: true,
  } satisfies GetManySubaccountsRequest;

  try {
    const data = await api.getManySubaccounts(body);
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
| **nocache** | `boolean` |  | [Optional] [Defaults to `false`] |

### Return type

[**Subaccounts2Response**](Subaccounts2Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | response with subaccount definitions |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getPaginatedPositions

> Array&lt;IndividualPosition&gt; getPaginatedPositions(accountId, pageId, model, sort, direction, waitForSecDef)

Account Positions

Get all positions in an account.

### Example

```ts
import {
  Configuration,
  TradingPortfolioApi,
} from 'bezant-client';
import type { GetPaginatedPositionsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingPortfolioApi();

  const body = {
    // string
    accountId: DU123456,
    // number
    pageId: 1,
    // any (optional)
    model: ...,
    // any (optional)
    sort: ...,
    // any (optional)
    direction: ...,
    // boolean (optional)
    waitForSecDef: true,
  } satisfies GetPaginatedPositionsRequest;

  try {
    const data = await api.getPaginatedPositions(body);
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
| **pageId** | `number` |  | [Defaults to `undefined`] |
| **model** | `any` |  | [Optional] [Defaults to `undefined`] |
| **sort** | `any` |  | [Optional] [Defaults to `undefined`] |
| **direction** | `any` |  | [Optional] [Defaults to `undefined`] |
| **waitForSecDef** | `boolean` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**Array&lt;IndividualPosition&gt;**](IndividualPosition.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | positions |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getPortfolioLedger

> { [key: string]: LedgerValue; } getPortfolioLedger(accountId)

Account Ledger

Get the given account\&#39;s ledger data detailing its balances by currency.

### Example

```ts
import {
  Configuration,
  TradingPortfolioApi,
} from 'bezant-client';
import type { GetPortfolioLedgerRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingPortfolioApi();

  const body = {
    // string
    accountId: DU123456,
  } satisfies GetPortfolioLedgerRequest;

  try {
    const data = await api.getPortfolioLedger(body);
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

[**{ [key: string]: LedgerValue; }**](LedgerValue.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | ledger |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getPortfolioMetadata

> AccountAttributes getPortfolioMetadata(accountId)

Account Attributes

Get a single account\&#39;s attributes and capabilities.

### Example

```ts
import {
  Configuration,
  TradingPortfolioApi,
} from 'bezant-client';
import type { GetPortfolioMetadataRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingPortfolioApi();

  const body = {
    // string
    accountId: DU123456,
  } satisfies GetPortfolioMetadataRequest;

  try {
    const data = await api.getPortfolioMetadata(body);
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

[**AccountAttributes**](AccountAttributes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | An account\&#39;s attributes |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getPortfolioSummary

> PortfolioSummary getPortfolioSummary(accountId)

Account Portfolio Summary

Returns detailed summary of account values, by segment where appropriate.

### Example

```ts
import {
  Configuration,
  TradingPortfolioApi,
} from 'bezant-client';
import type { GetPortfolioSummaryRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingPortfolioApi();

  const body = {
    // string
    accountId: DU123456,
  } satisfies GetPortfolioSummaryRequest;

  try {
    const data = await api.getPortfolioSummary(body);
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

[**PortfolioSummary**](PortfolioSummary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | response with summary definitions |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getPositionByConid

> Array&lt;IndividualPosition&gt; getPositionByConid(accountid, conid)

Account Position In An Instrument

Get position for a given instrument in a single account.

### Example

```ts
import {
  Configuration,
  TradingPortfolioApi,
} from 'bezant-client';
import type { GetPositionByConidRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingPortfolioApi();

  const body = {
    // string
    accountid: DU123456,
    // number
    conid: 265598,
  } satisfies GetPositionByConidRequest;

  try {
    const data = await api.getPositionByConid(body);
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
| **accountid** | `string` |  | [Defaults to `undefined`] |
| **conid** | `number` |  | [Defaults to `undefined`] |

### Return type

[**Array&lt;IndividualPosition&gt;**](IndividualPosition.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Position details by conid |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getUncachedPositions

> Array&lt;Portfolio2PositionsInner&gt; getUncachedPositions(accountId, model, direction)

Account Positions (NEW)

Returns a list of positions for the given account. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint. This endpoint provides near-real time updates and removes caching otherwise found in the /portfolio/{accountId}/positions/{pageId} endpoint. 

### Example

```ts
import {
  Configuration,
  TradingPortfolioApi,
} from 'bezant-client';
import type { GetUncachedPositionsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingPortfolioApi();

  const body = {
    // string
    accountId: DU123456,
    // string (optional)
    model: Primary_Model_Groups,
    // 'a' | 'd' (optional)
    direction: direction_example,
  } satisfies GetUncachedPositionsRequest;

  try {
    const data = await api.getUncachedPositions(body);
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
| **model** | `string` |  | [Optional] [Defaults to `undefined`] |
| **direction** | `a`, `d` |  | [Optional] [Defaults to `&#39;a&#39;`] [Enum: a, d] |

### Return type

[**Array&lt;Portfolio2PositionsInner&gt;**](Portfolio2PositionsInner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | response with position details |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## invalidatePositionCache

> InvalidatePositionCache200Response invalidatePositionCache(accountId)

Refresh Position Cache

Instructs IB to discard cached portfolio positions for a given account, so that the next request for positions delivers freshly obtained data.

### Example

```ts
import {
  Configuration,
  TradingPortfolioApi,
} from 'bezant-client';
import type { InvalidatePositionCacheRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingPortfolioApi();

  const body = {
    // string
    accountId: DU123456,
  } satisfies InvalidatePositionCacheRequest;

  try {
    const data = await api.invalidatePositionCache(body);
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

[**InvalidatePositionCache200Response**](InvalidatePositionCache200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | status of invalidation |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

