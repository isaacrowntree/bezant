# TradingSessionApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**getBrokerageStatus**](TradingSessionApi.md#getbrokeragestatus) | **POST** /iserver/auth/status | Brokerage Session Status |
| [**getSessionToken**](TradingSessionApi.md#getsessiontoken) | **POST** /tickle | Brokerage Keep-Alive Ping |
| [**getSessionValidation**](TradingSessionApi.md#getsessionvalidation) | **GET** /sso/validate | Validate SSO Web API Session |
| [**initializeSession**](TradingSessionApi.md#initializesession) | **POST** /iserver/auth/ssodh/init | Initialize Brokerage Session |
| [**logout**](TradingSessionApi.md#logout) | **POST** /logout | Terminate Web API Session |



## getBrokerageStatus

> BrokerageSessionStatus getBrokerageStatus()

Brokerage Session Status

Current Authentication status to the Brokerage system. Market Data and Trading is not possible if not authenticated.

### Example

```ts
import {
  Configuration,
  TradingSessionApi,
} from 'bezant-client';
import type { GetBrokerageStatusRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingSessionApi();

  try {
    const data = await api.getBrokerageStatus();
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

[**BrokerageSessionStatus**](BrokerageSessionStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Detailed status of the Brokerage session |  -  |
| **401** | Invalid or expired access token |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getSessionToken

> TickleResponse getSessionToken()

Brokerage Keep-Alive Ping

If the gateway has not received any requests for several minutes an open session will automatically timeout. The tickle endpoint pings the server to prevent the session from ending. It is expected to call this endpoint approximately every 60 seconds to maintain the connection to the brokerage session.

### Example

```ts
import {
  Configuration,
  TradingSessionApi,
} from 'bezant-client';
import type { GetSessionTokenRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingSessionApi();

  try {
    const data = await api.getSessionToken();
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

[**TickleResponse**](TickleResponse.md)

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

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getSessionValidation

> SsoValidateResponse getSessionValidation()

Validate SSO Web API Session

Validates the current session for the SSO user.

### Example

```ts
import {
  Configuration,
  TradingSessionApi,
} from 'bezant-client';
import type { GetSessionValidationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingSessionApi();

  try {
    const data = await api.getSessionValidation();
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

[**SsoValidateResponse**](SsoValidateResponse.md)

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

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## initializeSession

> BrokerageSessionStatus initializeSession(brokerageSessionInitRequest)

Initialize Brokerage Session

After retrieving the access token and subsequent Live Session Token, customers can initialize their brokerage session with the ssodh/init endpoint.

### Example

```ts
import {
  Configuration,
  TradingSessionApi,
} from 'bezant-client';
import type { InitializeSessionRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingSessionApi();

  const body = {
    // BrokerageSessionInitRequest
    brokerageSessionInitRequest: ...,
  } satisfies InitializeSessionRequest;

  try {
    const data = await api.initializeSession(body);
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
| **brokerageSessionInitRequest** | [BrokerageSessionInitRequest](BrokerageSessionInitRequest.md) |  | |

### Return type

[**BrokerageSessionStatus**](BrokerageSessionStatus.md)

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


## logout

> Logout200Response logout()

Terminate Web API Session

Logs the user out of the gateway session. Any further activity requires re-authentication. Discard client-side cookies upon logout. 

### Example

```ts
import {
  Configuration,
  TradingSessionApi,
} from 'bezant-client';
import type { LogoutRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingSessionApi();

  try {
    const data = await api.logout();
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

[**Logout200Response**](Logout200Response.md)

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

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

