# TradingPortfolioAnalystApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**getPerformanceAllPeriods**](TradingPortfolioAnalystApi.md#getperformanceallperiodsoperation) | **POST** /pa/allperiods | Account Performance (All Time Periods) |
| [**getSinglePerformancePeriod**](TradingPortfolioAnalystApi.md#getsingleperformanceperiodoperation) | **POST** /pa/performance | Account Performance |
| [**getTransactions**](TradingPortfolioAnalystApi.md#gettransactionsoperation) | **POST** /pa/transactions | Transaction History |



## getPerformanceAllPeriods

> DetailedContractInformation getPerformanceAllPeriods(getPerformanceAllPeriodsRequest, param)

Account Performance (All Time Periods)

Returns the performance (MTM) for the given accounts, if more than one account is passed, the result is consolidated.

### Example

```ts
import {
  Configuration,
  TradingPortfolioAnalystApi,
} from 'bezant-client';
import type { GetPerformanceAllPeriodsOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingPortfolioAnalystApi();

  const body = {
    // GetPerformanceAllPeriodsRequest
    getPerformanceAllPeriodsRequest: ...,
    // string (optional)
    param: param_example,
  } satisfies GetPerformanceAllPeriodsOperationRequest;

  try {
    const data = await api.getPerformanceAllPeriods(body);
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
| **getPerformanceAllPeriodsRequest** | [GetPerformanceAllPeriodsRequest](GetPerformanceAllPeriodsRequest.md) |  | |
| **param** | `string` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**DetailedContractInformation**](DetailedContractInformation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | An array of objects detailing contract information. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getSinglePerformancePeriod

> PerformanceResponse getSinglePerformancePeriod(getSinglePerformancePeriodRequest)

Account Performance

Returns the performance (MTM) for the given accounts, if more than one account is passed, the result is consolidated.

### Example

```ts
import {
  Configuration,
  TradingPortfolioAnalystApi,
} from 'bezant-client';
import type { GetSinglePerformancePeriodOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingPortfolioAnalystApi();

  const body = {
    // GetSinglePerformancePeriodRequest
    getSinglePerformancePeriodRequest: ...,
  } satisfies GetSinglePerformancePeriodOperationRequest;

  try {
    const data = await api.getSinglePerformancePeriod(body);
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
| **getSinglePerformancePeriodRequest** | [GetSinglePerformancePeriodRequest](GetSinglePerformancePeriodRequest.md) |  | |

### Return type

[**PerformanceResponse**](PerformanceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | An array of objects detailing contract information. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getTransactions

> TransactionsResponse getTransactions(getTransactionsRequest)

Transaction History

Transaction history for a given number of conids and accounts. Types of transactions include dividend payments, buy and sell transactions, transfers.

### Example

```ts
import {
  Configuration,
  TradingPortfolioAnalystApi,
} from 'bezant-client';
import type { GetTransactionsOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingPortfolioAnalystApi();

  const body = {
    // GetTransactionsRequest
    getTransactionsRequest: ...,
  } satisfies GetTransactionsOperationRequest;

  try {
    const data = await api.getTransactions(body);
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
| **getTransactionsRequest** | [GetTransactionsRequest](GetTransactionsRequest.md) |  | |

### Return type

[**TransactionsResponse**](TransactionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | An array of objects detailing contract information. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

