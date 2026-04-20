# TradingOAuth10aApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**reqAccessToken**](TradingOAuth10aApi.md#reqaccesstoken) | **POST** /oauth/access_token | Generate An Access Token |
| [**reqLiveSessionToken**](TradingOAuth10aApi.md#reqlivesessiontoken) | **POST** /oauth/live_session_token | Generate A Live Session Token |
| [**reqTempToken**](TradingOAuth10aApi.md#reqtemptoken) | **POST** /oauth/request_token | Obtain A Request Token |



## reqAccessToken

> ReqAccessToken200Response reqAccessToken(authorization)

Generate An Access Token

Request an access token for the IB username that has granted authorization to the consumer.

### Example

```ts
import {
  Configuration,
  TradingOAuth10aApi,
} from 'bezant-client';
import type { ReqAccessTokenRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingOAuth10aApi();

  const body = {
    // string | OAuth 1.0a authorization request header for request to /access_token endpoint. (optional)
    authorization: OAuth oauth_verifier="1e1dc5666e87ca5a18e0",oauth_token="e0d75b4c5c1d2c0f2af7",oauth_consumer_key="TESTCONS",oauth_nonce="v235...456h",oauth_signature="af1%252...0nd2",oauth_signature_method="RSA-SHA256",oauth_timestamp="1714489450",realm="test_realm",
  } satisfies ReqAccessTokenRequest;

  try {
    const data = await api.reqAccessToken(body);
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
| **authorization** | `string` | OAuth 1.0a authorization request header for request to /access_token endpoint. | [Optional] [Defaults to `undefined`] |

### Return type

[**ReqAccessToken200Response**](ReqAccessToken200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Success response with permanent OAuth access token |  -  |
| **401** | Invalid or expired access token |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## reqLiveSessionToken

> ReqLiveSessionToken200Response reqLiveSessionToken(authorization)

Generate A Live Session Token

Generate a Live Session Token shared secret and gain access to Web API.

### Example

```ts
import {
  Configuration,
  TradingOAuth10aApi,
} from 'bezant-client';
import type { ReqLiveSessionTokenRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingOAuth10aApi();

  const body = {
    // string | OAuth 1.0a authorization request header for request to /live_session_token endpoint. (optional)
    authorization: OAuth diffie_hellman_challenge="b393...g6f3",oauth_token="56786fc07bcbabc4584",oauth_consumer_key="TESTCONS",oauth_nonce="v235...456h",oauth_signature="af1%252...0nd2",oauth_signature_method="RSA-SHA256",oauth_timestamp="1714489460",realm="test_realm",
  } satisfies ReqLiveSessionTokenRequest;

  try {
    const data = await api.reqLiveSessionToken(body);
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
| **authorization** | `string` | OAuth 1.0a authorization request header for request to /live_session_token endpoint. | [Optional] [Defaults to `undefined`] |

### Return type

[**ReqLiveSessionToken200Response**](ReqLiveSessionToken200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Success response with Diffie-Hellman challenge and Signature value |  -  |
| **401** | Invalid or expired access token |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## reqTempToken

> ReqTempToken200Response reqTempToken(authorization)

Obtain A Request Token

Request a temporary token as a third party to begin the OAuth 1.0a authorization workflow.

### Example

```ts
import {
  Configuration,
  TradingOAuth10aApi,
} from 'bezant-client';
import type { ReqTempTokenRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingOAuth10aApi();

  const body = {
    // string | OAuth 1.0a authorization request header for request to /request_token endpoint. (optional)
    authorization: OAuth oauth_callback="oob",oauth_consumer_key="TESTCONS",oauth_nonce="b249...8f57",oauth_signature="41Sx%252...ZYZ2",oauth_signature_method="RSA-SHA256",oauth_timestamp="1714489440",realm="test_realm",
  } satisfies ReqTempTokenRequest;

  try {
    const data = await api.reqTempToken(body);
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
| **authorization** | `string` | OAuth 1.0a authorization request header for request to /request_token endpoint. | [Optional] [Defaults to `undefined`] |

### Return type

[**ReqTempToken200Response**](ReqTempToken200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Success response with temporary OAuth access token |  -  |
| **401** | Invalid or expired access token |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

