# TradingWatchlistsApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**deleteWatchlist**](TradingWatchlistsApi.md#deletewatchlist) | **DELETE** /iserver/watchlist | Delete A Saved Watchlist |
| [**getAllWatchlists**](TradingWatchlistsApi.md#getallwatchlists) | **GET** /iserver/watchlists | Return All Saved Watchlists |
| [**getSpecificWatchlist**](TradingWatchlistsApi.md#getspecificwatchlist) | **GET** /iserver/watchlist | Return A Single Saved Watchlist |
| [**postNewWatchlist**](TradingWatchlistsApi.md#postnewwatchlistoperation) | **POST** /iserver/watchlist | Create A Watchlist |



## deleteWatchlist

> WatchlistDeleteSuccess deleteWatchlist(id)

Delete A Saved Watchlist

Delete a specified watchlist from the username\&#39;s settings.

### Example

```ts
import {
  Configuration,
  TradingWatchlistsApi,
} from 'bezant-client';
import type { DeleteWatchlistRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingWatchlistsApi();

  const body = {
    // string | Watchlist ID of the watchlist to be deleted.
    id: 1234,
  } satisfies DeleteWatchlistRequest;

  try {
    const data = await api.deleteWatchlist(body);
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
| **id** | `string` | Watchlist ID of the watchlist to be deleted. | [Defaults to `undefined`] |

### Return type

[**WatchlistDeleteSuccess**](WatchlistDeleteSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successful deletion of specified watchlist. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getAllWatchlists

> WatchlistsResponse getAllWatchlists(sC)

Return All Saved Watchlists

Returns all saved watchlists stored on IB backend for the username in use in the current Web API session.

### Example

```ts
import {
  Configuration,
  TradingWatchlistsApi,
} from 'bezant-client';
import type { GetAllWatchlistsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingWatchlistsApi();

  const body = {
    // 'USER_WATCHLIST' | Can only be used with value USER_WATCHLIST, which returns only user-created watchlists and excludes those created by IB. (optional)
    sC: USER_WATCHLIST,
  } satisfies GetAllWatchlistsRequest;

  try {
    const data = await api.getAllWatchlists(body);
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
| **sC** | `USER_WATCHLIST` | Can only be used with value USER_WATCHLIST, which returns only user-created watchlists and excludes those created by IB. | [Optional] [Defaults to `undefined`] [Enum: USER_WATCHLIST] |

### Return type

[**WatchlistsResponse**](WatchlistsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Historical data query successfully returned data. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getSpecificWatchlist

> SingleWatchlist getSpecificWatchlist(id)

Return A Single Saved Watchlist

Retrieve details of a single watchlist stored in the username\&#39;s settings.

### Example

```ts
import {
  Configuration,
  TradingWatchlistsApi,
} from 'bezant-client';
import type { GetSpecificWatchlistRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingWatchlistsApi();

  const body = {
    // string | Watchlist ID of the requested watchlist.
    id: 1234,
  } satisfies GetSpecificWatchlistRequest;

  try {
    const data = await api.getSpecificWatchlist(body);
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
| **id** | `string` | Watchlist ID of the requested watchlist. | [Defaults to `undefined`] |

### Return type

[**SingleWatchlist**](SingleWatchlist.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successful deletion of specified watchlist. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postNewWatchlist

> PostNewWatchlist200Response postNewWatchlist(postNewWatchlistRequest)

Create A Watchlist

Create a named watchlist by submitting a set of conids.

### Example

```ts
import {
  Configuration,
  TradingWatchlistsApi,
} from 'bezant-client';
import type { PostNewWatchlistOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingWatchlistsApi();

  const body = {
    // PostNewWatchlistRequest | Watchlist contents.
    postNewWatchlistRequest: ...,
  } satisfies PostNewWatchlistOperationRequest;

  try {
    const data = await api.postNewWatchlist(body);
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
| **postNewWatchlistRequest** | [PostNewWatchlistRequest](PostNewWatchlistRequest.md) | Watchlist contents. | |

### Return type

[**PostNewWatchlist200Response**](PostNewWatchlist200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Watchlist creation successful. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

