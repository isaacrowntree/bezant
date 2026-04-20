# TradingFYIsAndNotificationsApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**deleteFyiDevice**](TradingFYIsAndNotificationsApi.md#deletefyidevice) | **DELETE** /fyi/deliveryoptions/{deviceId} | Delete A Device |
| [**getAllFyis**](TradingFYIsAndNotificationsApi.md#getallfyis) | **GET** /fyi/notifications | List All Notifications |
| [**getFyiDelivery**](TradingFYIsAndNotificationsApi.md#getfyidelivery) | **GET** /fyi/deliveryoptions | Get Delivery Options |
| [**getFyiDisclaimerss**](TradingFYIsAndNotificationsApi.md#getfyidisclaimerss) | **GET** /fyi/disclaimer/{typecode} | Get Disclaimers By FYI Type |
| [**getFyiSettings**](TradingFYIsAndNotificationsApi.md#getfyisettings) | **GET** /fyi/settings | Get Notification Settings |
| [**getUnreadFyis**](TradingFYIsAndNotificationsApi.md#getunreadfyis) | **GET** /fyi/unreadnumber | Get Number Of Unread Notifications |
| [**modifyFyiDelivery**](TradingFYIsAndNotificationsApi.md#modifyfyidelivery) | **POST** /fyi/deliveryoptions/device | Toggle Delivery To A Device |
| [**modifyFyiEmails**](TradingFYIsAndNotificationsApi.md#modifyfyiemails) | **PUT** /fyi/deliveryoptions/email | Toggle Email Delivery |
| [**modifyFyiNotification**](TradingFYIsAndNotificationsApi.md#modifyfyinotificationoperation) | **POST** /fyi/settings/{typecode} | Modify FYI Notifications |
| [**readFyiDisclaimer**](TradingFYIsAndNotificationsApi.md#readfyidisclaimer) | **PUT** /fyi/disclaimer/{typecode} | Mark FYI Disclaimer Read |
| [**readFyiNotification**](TradingFYIsAndNotificationsApi.md#readfyinotification) | **PUT** /fyi/notifications/{notificationID} | Mark Notification Read |



## deleteFyiDevice

> deleteFyiDevice(deviceId)

Delete A Device

Delete a specific device from our saved list of notification devices.

### Example

```ts
import {
  Configuration,
  TradingFYIsAndNotificationsApi,
} from 'bezant-client';
import type { DeleteFyiDeviceRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFYIsAndNotificationsApi();

  const body = {
    // any
    deviceId: ...,
  } satisfies DeleteFyiDeviceRequest;

  try {
    const data = await api.deleteFyiDevice(body);
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
| **deviceId** | `any` |  | [Defaults to `undefined`] |

### Return type

`void` (Empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | No response message is returned. Instead, you will only receive an empty string with a 200 OK status code indicating a successfully deleted account. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getAllFyis

> Array&lt;NotificationsInner&gt; getAllFyis(max, include, exclude, id)

List All Notifications

Get a list of available notifications.

### Example

```ts
import {
  Configuration,
  TradingFYIsAndNotificationsApi,
} from 'bezant-client';
import type { GetAllFyisRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFYIsAndNotificationsApi();

  const body = {
    // number
    max: 10,
    // any (optional)
    include: ...,
    // any (optional)
    exclude: ...,
    // any (optional)
    id: ...,
  } satisfies GetAllFyisRequest;

  try {
    const data = await api.getAllFyis(body);
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
| **max** | `number` |  | [Defaults to `undefined`] |
| **include** | `any` |  | [Optional] [Defaults to `undefined`] |
| **exclude** | `any` |  | [Optional] [Defaults to `undefined`] |
| **id** | `any` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**Array&lt;NotificationsInner&gt;**](NotificationsInner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successfully enabled or disabled your email notifications. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getFyiDelivery

> DeliveryOptions getFyiDelivery()

Get Delivery Options

Options for sending fyis to email and other devices.

### Example

```ts
import {
  Configuration,
  TradingFYIsAndNotificationsApi,
} from 'bezant-client';
import type { GetFyiDeliveryRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFYIsAndNotificationsApi();

  try {
    const data = await api.getFyiDelivery();
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

[**DeliveryOptions**](DeliveryOptions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successfully retrieve preset details |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getFyiDisclaimerss

> DisclaimerInfo getFyiDisclaimerss(typecode)

Get Disclaimers By FYI Type

Receive additional disclaimers based on the specified typecode.

### Example

```ts
import {
  Configuration,
  TradingFYIsAndNotificationsApi,
} from 'bezant-client';
import type { GetFyiDisclaimerssRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFYIsAndNotificationsApi();

  const body = {
    // Typecodes
    typecode: ...,
  } satisfies GetFyiDisclaimerssRequest;

  try {
    const data = await api.getFyiDisclaimerss(body);
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
| **typecode** | `Typecodes` |  | [Defaults to `undefined`] [Enum: BA, CA, DA, EA, MF, OE, PR, SE, SG, SM, T2, TO, UA, M8, PS, DL, PT, CB, MS, TD, ST, TI, CT] |

### Return type

[**DisclaimerInfo**](DisclaimerInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successfully disclaimer details |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getFyiSettings

> Array&lt;FyiSettingsInner&gt; getFyiSettings()

Get Notification Settings

Return the current choices of subscriptions for notifications.

### Example

```ts
import {
  Configuration,
  TradingFYIsAndNotificationsApi,
} from 'bezant-client';
import type { GetFyiSettingsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFYIsAndNotificationsApi();

  try {
    const data = await api.getFyiSettings();
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

[**Array&lt;FyiSettingsInner&gt;**](FyiSettingsInner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Indicates data is being returned successfully. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getUnreadFyis

> GetUnreadFyis200Response getUnreadFyis()

Get Number Of Unread Notifications

Returns the total number of unread notifications

### Example

```ts
import {
  Configuration,
  TradingFYIsAndNotificationsApi,
} from 'bezant-client';
import type { GetUnreadFyisRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFYIsAndNotificationsApi();

  try {
    const data = await api.getUnreadFyis();
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

[**GetUnreadFyis200Response**](GetUnreadFyis200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Indicates data is being returned successfully. |  -  |
| **401** | Invalid or expired access token |  -  |
| **423** | Return if called too frequently. Should not be called more than 1 time in 5 minutes |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## modifyFyiDelivery

> FyiVT modifyFyiDelivery(fyiEnableDeviceOption)

Toggle Delivery To A Device

Choose whether a particular device is enabled or disabled.

### Example

```ts
import {
  Configuration,
  TradingFYIsAndNotificationsApi,
} from 'bezant-client';
import type { ModifyFyiDeliveryRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFYIsAndNotificationsApi();

  const body = {
    // FyiEnableDeviceOption
    fyiEnableDeviceOption: ...,
  } satisfies ModifyFyiDeliveryRequest;

  try {
    const data = await api.modifyFyiDelivery(body);
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
| **fyiEnableDeviceOption** | [FyiEnableDeviceOption](FyiEnableDeviceOption.md) |  | |

### Return type

[**FyiVT**](FyiVT.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successfully retrieve preset details |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## modifyFyiEmails

> FyiVT modifyFyiEmails(enabled)

Toggle Email Delivery

Enable or disable your account\&#39;s primary email to receive notifications.

### Example

```ts
import {
  Configuration,
  TradingFYIsAndNotificationsApi,
} from 'bezant-client';
import type { ModifyFyiEmailsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFYIsAndNotificationsApi();

  const body = {
    // any
    enabled: ...,
  } satisfies ModifyFyiEmailsRequest;

  try {
    const data = await api.modifyFyiEmails(body);
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
| **enabled** | `any` |  | [Defaults to `undefined`] |

### Return type

[**FyiVT**](FyiVT.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successfully enabled or disabled your email notifications. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## modifyFyiNotification

> FyiVT modifyFyiNotification(typecode, modifyFyiNotificationRequest)

Modify FYI Notifications

Enable or disable group of notifications by the specific typecode.

### Example

```ts
import {
  Configuration,
  TradingFYIsAndNotificationsApi,
} from 'bezant-client';
import type { ModifyFyiNotificationOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFYIsAndNotificationsApi();

  const body = {
    // Typecodes
    typecode: ...,
    // ModifyFyiNotificationRequest
    modifyFyiNotificationRequest: ...,
  } satisfies ModifyFyiNotificationOperationRequest;

  try {
    const data = await api.modifyFyiNotification(body);
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
| **typecode** | `Typecodes` |  | [Defaults to `undefined`] [Enum: BA, CA, DA, EA, MF, OE, PR, SE, SG, SM, T2, TO, UA, M8, PS, DL, PT, CB, MS, TD, ST, TI, CT] |
| **modifyFyiNotificationRequest** | [ModifyFyiNotificationRequest](ModifyFyiNotificationRequest.md) |  | |

### Return type

[**FyiVT**](FyiVT.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successfully retrieve preset details |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## readFyiDisclaimer

> FyiVT readFyiDisclaimer(typecode)

Mark FYI Disclaimer Read

Mark a specific disclaimer message as read.

### Example

```ts
import {
  Configuration,
  TradingFYIsAndNotificationsApi,
} from 'bezant-client';
import type { ReadFyiDisclaimerRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFYIsAndNotificationsApi();

  const body = {
    // Typecodes
    typecode: ...,
  } satisfies ReadFyiDisclaimerRequest;

  try {
    const data = await api.readFyiDisclaimer(body);
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
| **typecode** | `Typecodes` |  | [Defaults to `undefined`] [Enum: BA, CA, DA, EA, MF, OE, PR, SE, SG, SM, T2, TO, UA, M8, PS, DL, PT, CB, MS, TD, ST, TI, CT] |

### Return type

[**FyiVT**](FyiVT.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successfully marked as read |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## readFyiNotification

> NotificationReadAcknowledge readFyiNotification(notificationID)

Mark Notification Read

Mark a particular notification message as read or unread.

### Example

```ts
import {
  Configuration,
  TradingFYIsAndNotificationsApi,
} from 'bezant-client';
import type { ReadFyiNotificationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFYIsAndNotificationsApi();

  const body = {
    // any
    notificationID: ...,
  } satisfies ReadFyiNotificationRequest;

  try {
    const data = await api.readFyiNotification(body);
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
| **notificationID** | `any` |  | [Defaults to `undefined`] |

### Return type

[**NotificationReadAcknowledge**](NotificationReadAcknowledge.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successfully enabled or disabled your email notifications. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

