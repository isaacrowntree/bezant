# TradingAlertsApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**activateAlert**](TradingAlertsApi.md#activatealert) | **POST** /iserver/account/{accountId}/alert/activate | Activate Or Deactivate An Alert |
| [**createAlert**](TradingAlertsApi.md#createalert) | **POST** /iserver/account/{accountId}/alert | Create Or Modify Alert |
| [**deleteAlert**](TradingAlertsApi.md#deletealert) | **DELETE** /iserver/account/{accountId}/alert/{alertId} | Delete An Alert |
| [**getAlertDetails**](TradingAlertsApi.md#getalertdetails) | **GET** /iserver/account/alert/{alertId} | Details Of A Specific Alert |
| [**getAllAlerts**](TradingAlertsApi.md#getallalerts) | **GET** /iserver/account/{accountId}/alerts | List All Alerts |
| [**getMtaDetails**](TradingAlertsApi.md#getmtadetails) | **GET** /iserver/account/mta | Details Of A Mobile Trading Alert |



## activateAlert

> AlertActivationResponse activateAlert(accountId, alertActivationRequest)

Activate Or Deactivate An Alert

Activate or Deactivate existing alerts created for this account. This does not delete alerts, but disables notifications until reactivated.

### Example

```ts
import {
  Configuration,
  TradingAlertsApi,
} from 'bezant-client';
import type { ActivateAlertRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingAlertsApi();

  const body = {
    // string
    accountId: U1234567,
    // AlertActivationRequest
    alertActivationRequest: {"alertActive":1,"alertId":9876543210},
  } satisfies ActivateAlertRequest;

  try {
    const data = await api.activateAlert(body);
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
| **alertActivationRequest** | [AlertActivationRequest](AlertActivationRequest.md) |  | |

### Return type

[**AlertActivationResponse**](AlertActivationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | An object containing details about the activated/deactivated alert. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | Internal Server Error; unable to process incoming request due to invalid data in it |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## createAlert

> AlertCreationResponse createAlert(accountId, alertCreationRequest)

Create Or Modify Alert

Endpoint used to create a new alert, or modify an existing alert.

### Example

```ts
import {
  Configuration,
  TradingAlertsApi,
} from 'bezant-client';
import type { CreateAlertRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingAlertsApi();

  const body = {
    // string
    accountId: U1234567,
    // AlertCreationRequest
    alertCreationRequest: {"alertMessage":"AAPL Price Drop!","alertName":"AAPL_Price","alertRepeatable":0,"conditions":[{"conidex":"265598@NYSE","logicBind":"a","operator":">=","timeZone":"US/Eastern","triggerMethod":"0","type":1,"value":"500"}],"email":"jonh.smith@example.com","expireTime":"20231231-12:00:00","iTWSOrdersOnly":0,"outsideRth":0,"sendMessage":1,"showPopup":1,"tif":"GTC"},
  } satisfies CreateAlertRequest;

  try {
    const data = await api.createAlert(body);
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
| **alertCreationRequest** | [AlertCreationRequest](AlertCreationRequest.md) |  | |

### Return type

[**AlertCreationResponse**](AlertCreationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | An object containing valid accounts and the account properties regarding trading access. This endpoint is also used to confirm account validation. |  -  |
| **400** | bad request; body is empty |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | Internal Server Error. Unable to process request if incoming values are not valid. For example operator is \&quot;abc\&quot; Or if modification request contains unmodified fields  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## deleteAlert

> AlertDeletionResponse deleteAlert(accountId, alertId, body)

Delete An Alert

Permanently delete an existing alert. Deleting an MTA alert will reset it to the default state.

### Example

```ts
import {
  Configuration,
  TradingAlertsApi,
} from 'bezant-client';
import type { DeleteAlertRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingAlertsApi();

  const body = {
    // string
    accountId: U1234567,
    // string
    alertId: 9876543210,
    // object
    body: Object,
  } satisfies DeleteAlertRequest;

  try {
    const data = await api.deleteAlert(body);
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
| **alertId** | `string` |  | [Defaults to `undefined`] |
| **body** | `object` |  | |

### Return type

[**AlertDeletionResponse**](AlertDeletionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | An object containing details on the deleted endpoint. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | Internal Server Error; Unable to delete alert in case when provided alert id doesn\&#39;t exist |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getAlertDetails

> AlertDetails getAlertDetails(alertId, type)

Details Of A Specific Alert

Request details of a specific alert by providing the assigned alertId Id.

### Example

```ts
import {
  Configuration,
  TradingAlertsApi,
} from 'bezant-client';
import type { GetAlertDetailsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingAlertsApi();

  const body = {
    // number
    alertId: 789,
    // 'Q'
    type: type_example,
  } satisfies GetAlertDetailsRequest;

  try {
    const data = await api.getAlertDetails(body);
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
| **alertId** | `number` |  | [Defaults to `undefined`] |
| **type** | `Q` |  | [Defaults to `undefined`] [Enum: Q] |

### Return type

[**AlertDetails**](AlertDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | An object containing all unique details of the specified alert. |  -  |
| **400** | bad request if orderId is empty or type is invalid |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | orderId is not parsable; unable to process request |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getAllAlerts

> Set&lt;Alert&gt; getAllAlerts(accountId)

List All Alerts

Retrieve a list of all alerts attached to the provided account.

### Example

```ts
import {
  Configuration,
  TradingAlertsApi,
} from 'bezant-client';
import type { GetAllAlertsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingAlertsApi();

  const body = {
    // string
    accountId: U1234567,
  } satisfies GetAllAlertsRequest;

  try {
    const data = await api.getAllAlerts(body);
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

[**Set&lt;Alert&gt;**](Alert.md)

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
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getMtaDetails

> AlertDetails getMtaDetails()

Details Of A Mobile Trading Alert

Retrieve information about your MTA alert. Each login user only has one mobile trading assistant (MTA) alert with it\&#39;s own unique tool id that cannot be changed. MTA alerts can not be created or deleted, only modified. When modified a new order Id is generated. 

### Example

```ts
import {
  Configuration,
  TradingAlertsApi,
} from 'bezant-client';
import type { GetMtaDetailsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingAlertsApi();

  try {
    const data = await api.getMtaDetails();
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

[**AlertDetails**](AlertDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | The alert description for the unique MTA alert on the account. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

