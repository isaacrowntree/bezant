# TradingEventContractsApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**getForecastCategories**](TradingEventContractsApi.md#getforecastcategories) | **GET** /forecast/category/tree | Event Contract Categories |
| [**getForecastContract**](TradingEventContractsApi.md#getforecastcontract) | **GET** /forecast/contract/details | Event Contract Details |
| [**getForecastMarkets**](TradingEventContractsApi.md#getforecastmarkets) | **GET** /forecast/contract/market | Provides All Contracts For Given Underlying Market. |
| [**getForecastRules**](TradingEventContractsApi.md#getforecastrules) | **GET** /forecast/contract/rules | Event Contract Rules |
| [**getForecastSchedule**](TradingEventContractsApi.md#getforecastschedule) | **GET** /forecast/contract/schedules | Event Contract Schedules |



## getForecastCategories

> CategoryTreeResponse getForecastCategories()

Event Contract Categories

Returns the category names, parent ids, and markets for Event Contracts.

### Example

```ts
import {
  Configuration,
  TradingEventContractsApi,
} from 'bezant-client';
import type { GetForecastCategoriesRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingEventContractsApi();

  try {
    const data = await api.getForecastCategories();
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

[**CategoryTreeResponse**](CategoryTreeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns a list of category identifiers and name to be used for more granular contract discovery. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getForecastContract

> ForecastDetailsResponse getForecastContract(conid)

Event Contract Details

Provides instrument details for the specific forecast contract.

### Example

```ts
import {
  Configuration,
  TradingEventContractsApi,
} from 'bezant-client';
import type { GetForecastContractRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingEventContractsApi();

  const body = {
    // string
    conid: conid_example,
  } satisfies GetForecastContractRequest;

  try {
    const data = await api.getForecastContract(body);
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
| **conid** | `string` |  | [Defaults to `undefined`] |

### Return type

[**ForecastDetailsResponse**](ForecastDetailsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns the expanded list of the event contract containing both Yes and No side identifier information. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getForecastMarkets

> ForecastMarketResponse getForecastMarkets(underlyingConid, exchange)

Provides All Contracts For Given Underlying Market.

Returns all high level contract details affiliated with the underlying market conid provided.

### Example

```ts
import {
  Configuration,
  TradingEventContractsApi,
} from 'bezant-client';
import type { GetForecastMarketsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingEventContractsApi();

  const body = {
    // string
    underlyingConid: underlyingConid_example,
    // string (optional)
    exchange: exchange_example,
  } satisfies GetForecastMarketsRequest;

  try {
    const data = await api.getForecastMarkets(body);
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
| **underlyingConid** | `string` |  | [Defaults to `undefined`] |
| **exchange** | `string` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**ForecastMarketResponse**](ForecastMarketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns the liquid and extended trading hours of the coming and prior trading days. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getForecastRules

> ForecastRulesResponse getForecastRules(conid)

Event Contract Rules

Provides trading rules for specific event contracts.

### Example

```ts
import {
  Configuration,
  TradingEventContractsApi,
} from 'bezant-client';
import type { GetForecastRulesRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingEventContractsApi();

  const body = {
    // string
    conid: conid_example,
  } satisfies GetForecastRulesRequest;

  try {
    const data = await api.getForecastRules(body);
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
| **conid** | `string` |  | [Defaults to `undefined`] |

### Return type

[**ForecastRulesResponse**](ForecastRulesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns the corresponding contracts rules. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getForecastSchedule

> ForecastSchedulesResponse getForecastSchedule(conid)

Event Contract Schedules

Provides forecast trading schedules.

### Example

```ts
import {
  Configuration,
  TradingEventContractsApi,
} from 'bezant-client';
import type { GetForecastScheduleRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingEventContractsApi();

  const body = {
    // string
    conid: conid_example,
  } satisfies GetForecastScheduleRequest;

  try {
    const data = await api.getForecastSchedule(body);
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
| **conid** | `string` |  | [Defaults to `undefined`] |

### Return type

[**ForecastSchedulesResponse**](ForecastSchedulesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns the liquid and extended trading hours of the coming and prior trading days. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

