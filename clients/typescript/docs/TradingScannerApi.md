# TradingScannerApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**getScannerParameters**](TradingScannerApi.md#getscannerparameters) | **GET** /iserver/scanner/params | Get Valid IServer Scanner Parameters |
| [**getScannerResults**](TradingScannerApi.md#getscannerresults) | **POST** /iserver/scanner/run | Run An IServer Market Scanner |



## getScannerParameters

> IserverScannerParams getScannerParameters()

Get Valid IServer Scanner Parameters

Returns an xml file containing all available parameters to be sent for the Iserver scanner request.

### Example

```ts
import {
  Configuration,
  TradingScannerApi,
} from 'bezant-client';
import type { GetScannerParametersRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingScannerApi();

  try {
    const data = await api.getScannerParameters();
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

[**IserverScannerParams**](IserverScannerParams.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | An array of objects detailing contract information. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getScannerResults

> IserverScannerRunResponse getScannerResults(iserverScannerRunRequest)

Run An IServer Market Scanner

Searches for contracts according to the filters specified in /iserver/scanner/params endpoint.

### Example

```ts
import {
  Configuration,
  TradingScannerApi,
} from 'bezant-client';
import type { GetScannerResultsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingScannerApi();

  const body = {
    // IserverScannerRunRequest
    iserverScannerRunRequest: {"filter":[{"code":"priceAbove","value":5}],"instrument":"STK","location":"STK.US.MAJOR","type":"TOP_TRADE_COUNT"},
  } satisfies GetScannerResultsRequest;

  try {
    const data = await api.getScannerResults(body);
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
| **iserverScannerRunRequest** | [IserverScannerRunRequest](IserverScannerRunRequest.md) |  | |

### Return type

[**IserverScannerRunResponse**](IserverScannerRunResponse.md)

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

