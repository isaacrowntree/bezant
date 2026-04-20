# TradingMarketDataApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**closeAllMdStreams**](TradingMarketDataApi.md#closeallmdstreams) | **GET** /iserver/marketdata/unsubscribeall | Close All Backend Data Streams |
| [**closeMdStream**](TradingMarketDataApi.md#closemdstreamoperation) | **POST** /iserver/marketdata/unsubscribe | Close A Backend Data Stream |
| [**getMdHistory**](TradingMarketDataApi.md#getmdhistory) | **GET** /iserver/marketdata/history | Historical OHLC Bar Data |
| [**getMdSnapshot**](TradingMarketDataApi.md#getmdsnapshot) | **GET** /iserver/marketdata/snapshot | Live Market Data Snapshot |



## closeAllMdStreams

> CloseAllMdStreams200Response closeAllMdStreams()

Close All Backend Data Streams

Instruct IServer to close all of its open backend data streams for all instruments.

### Example

```ts
import {
  Configuration,
  TradingMarketDataApi,
} from 'bezant-client';
import type { CloseAllMdStreamsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingMarketDataApi();

  try {
    const data = await api.closeAllMdStreams();
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

[**CloseAllMdStreams200Response**](CloseAllMdStreams200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Indicates a successful request to unsubscribe all streams. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## closeMdStream

> CloseMdStream200Response closeMdStream(closeMdStreamRequest)

Close A Backend Data Stream

Instruct IServer to close its backend stream for the instrument when real-time snapshots are no longer needed.

### Example

```ts
import {
  Configuration,
  TradingMarketDataApi,
} from 'bezant-client';
import type { CloseMdStreamOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingMarketDataApi();

  const body = {
    // CloseMdStreamRequest
    closeMdStreamRequest: ...,
  } satisfies CloseMdStreamOperationRequest;

  try {
    const data = await api.closeMdStream(body);
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
| **closeMdStreamRequest** | [CloseMdStreamRequest](CloseMdStreamRequest.md) |  | |

### Return type

[**CloseMdStream200Response**](CloseMdStream200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Acknowledges a successful request |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getMdHistory

> GetMdHistory200Response getMdHistory(conid, period, bar, exchange, outsideRth, startTime, direction, source)

Historical OHLC Bar Data

Request historical data for an instrument in the form of OHLC bars.

### Example

```ts
import {
  Configuration,
  TradingMarketDataApi,
} from 'bezant-client';
import type { GetMdHistoryRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingMarketDataApi();

  const body = {
    // number | IB contract ID of the requested instrument.
    conid: 265598,
    // string | A time duration away from startTime into the future to be divided into bars of the specified width. Supported Period Sizes   * `min` - Minutes   * `h` - Hour(s)   * `d` - Day(s)   * `w` - Week(s)   * `m` - Month(s)   * `y` - Year(s) 
    period: 6d,
    // string | The width of the bars into which the interval determined by period and startTime will be divided. It is not required that bar evenly divide period; partial bars can be returned. Supported Bar Sizes   * `S` - Second(s)   * `min` - Minute(s)   * `h` - Hour(s)   * `d` - Day(s)   * `w` - Week(s)   * `m` - Month(s) 
    bar: 5min,
    // string | Exchange (or SMART) from which data is requested. (optional)
    exchange: NASDAQ,
    // boolean | Indicates whether data outside of regular trading hours should be included in response. (optional)
    outsideRth: true,
    // string | A fixed UTC date-time reference point for the historical data request, from which the specified period extends. Format is YYYYMMDD-hh:mm:ss. If omitted, the current time is used, and direction must be omitted or 1. (optional)
    startTime: startTime_example,
    // -1 | 1 | Indicates whether data should begin or end at the start time.   * `-1` - Historical data will begin away from the start time, ending at the current time/startTime.   * `1` - Historical data begins at the start time, moving towards the current time. Only supported when startTime is included.  (optional)
    direction: 56,
    // string | The type of data to be returned in the historical bars. Supported Bar Sizes   * `Bid_Ask` - The OHLC bid/ask values.   * `Last` - The OHLC trade values.   * `Midpoint` - The OHLC of the Bid-Ask midpoint.  (optional)
    source: Midpoint,
  } satisfies GetMdHistoryRequest;

  try {
    const data = await api.getMdHistory(body);
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
| **conid** | `number` | IB contract ID of the requested instrument. | [Defaults to `undefined`] |
| **period** | `string` | A time duration away from startTime into the future to be divided into bars of the specified width. Supported Period Sizes   * &#x60;min&#x60; - Minutes   * &#x60;h&#x60; - Hour(s)   * &#x60;d&#x60; - Day(s)   * &#x60;w&#x60; - Week(s)   * &#x60;m&#x60; - Month(s)   * &#x60;y&#x60; - Year(s)  | [Defaults to `&#39;1d&#39;`] |
| **bar** | `string` | The width of the bars into which the interval determined by period and startTime will be divided. It is not required that bar evenly divide period; partial bars can be returned. Supported Bar Sizes   * &#x60;S&#x60; - Second(s)   * &#x60;min&#x60; - Minute(s)   * &#x60;h&#x60; - Hour(s)   * &#x60;d&#x60; - Day(s)   * &#x60;w&#x60; - Week(s)   * &#x60;m&#x60; - Month(s)  | [Defaults to `&#39;1min&#39;`] |
| **exchange** | `string` | Exchange (or SMART) from which data is requested. | [Optional] [Defaults to `undefined`] |
| **outsideRth** | `boolean` | Indicates whether data outside of regular trading hours should be included in response. | [Optional] [Defaults to `false`] |
| **startTime** | `string` | A fixed UTC date-time reference point for the historical data request, from which the specified period extends. Format is YYYYMMDD-hh:mm:ss. If omitted, the current time is used, and direction must be omitted or 1. | [Optional] [Defaults to `undefined`] |
| **direction** | `-1`, `1` | Indicates whether data should begin or end at the start time.   * &#x60;-1&#x60; - Historical data will begin away from the start time, ending at the current time/startTime.   * &#x60;1&#x60; - Historical data begins at the start time, moving towards the current time. Only supported when startTime is included.  | [Optional] [Defaults to `-1`] [Enum: -1, 1] |
| **source** | `string` | The type of data to be returned in the historical bars. Supported Bar Sizes   * &#x60;Bid_Ask&#x60; - The OHLC bid/ask values.   * &#x60;Last&#x60; - The OHLC trade values.   * &#x60;Midpoint&#x60; - The OHLC of the Bid-Ask midpoint.  | [Optional] [Defaults to `&#39;Last&#39;`] |

### Return type

[**GetMdHistory200Response**](GetMdHistory200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Historical data query successfully returned data. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getMdSnapshot

> Array&lt;IserverSnapshotInner&gt; getMdSnapshot(conids, fields)

Live Market Data Snapshot

Get Market Data for the given conid(s). A pre-flight request must be made prior to ever receiving data. For some fields, it may take more than a few moments to receive information. See response fields for a list of available fields that can be request via fields argument. The endpoint /iserver/accounts must be called prior to /iserver/marketdata/snapshot. For derivative contracts the endpoint /iserver/secdef/search must be called first. 

### Example

```ts
import {
  Configuration,
  TradingMarketDataApi,
} from 'bezant-client';
import type { GetMdSnapshotRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingMarketDataApi();

  const body = {
    // number
    conids: 265598,
    // MdFields (optional)
    fields: ...,
  } satisfies GetMdSnapshotRequest;

  try {
    const data = await api.getMdSnapshot(body);
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
| **conids** | `number` |  | [Defaults to `undefined`] |
| **fields** | `MdFields` |  | [Optional] [Defaults to `undefined`] [Enum: 31, 55, 58, 70, 71, 73, 74, 75, 76, 77, 78, 79, 80, 82, 83, 84, 85, 86, 87, 88, 6004, 6008, 6070, 6072, 6073, 6119, 6457, 6508, 6509, 7051, 7057, 7058, 7059, 7068, 7084, 7085, 7086, 7087, 7088, 7089, 7094, 7184, 7219, 7220, 7221, 7280, 7281, 7282, 7283, 7284, 7285, 7292, 7293, 7294, 7295, 7296, 7308, 7309, 7310, 7311, 7607, 7633, 7635, 7636, 7637, 7638, 7639, 7644, 7671, 7672, 7674, 7675, 7676, 7677, 7678, 7679, 7724, 7681, 7682, 7683, 7684, 7685, 7686, 7687, 7688, 7689, 7690, 7694, 7695, 7696, 7697, 7698, 7699, 7700, 7702, 7703, 7704, 7705, 7706, 7707, 7708, 7714, 7715, 7720, 7741, 7762, 7768, 7920, 7921, 7943] |

### Return type

[**Array&lt;IserverSnapshotInner&gt;**](IserverSnapshotInner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successfully requested market data |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

