# AuthorizationSSOSessionsApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**postGwApiV1SsoBrowserSessions**](AuthorizationSSOSessionsApi.md#postgwapiv1ssobrowsersessions) | **POST** /gw/api/v1/sso-browser-sessions | Create SSO Browser Session. |
| [**postGwApiV1SsoSessions**](AuthorizationSSOSessionsApi.md#postgwapiv1ssosessions) | **POST** /gw/api/v1/sso-sessions | Create A New SSO Session On Behalf Of An End-user. |



## postGwApiV1SsoBrowserSessions

> CreateBrowserSessionResponse postGwApiV1SsoBrowserSessions(authorization, body)

Create SSO Browser Session.

&lt;br&gt;**Scope**: &#x60;sso-browser-sessions.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AuthorizationSSOSessionsApi,
} from 'bezant-client';
import type { PostGwApiV1SsoBrowserSessionsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AuthorizationSSOSessionsApi();

  const body = {
    // string | Specifies the authorization header value (e.g., Bearer eyJ0eXAiOiJKV1...).
    authorization: Bearer eyJ0eXAiOiJKV1...,
    // string | Create browser session on behalf of end-user.
    body: body_example,
  } satisfies PostGwApiV1SsoBrowserSessionsRequest;

  try {
    const data = await api.postGwApiV1SsoBrowserSessions(body);
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
| **authorization** | `string` | Specifies the authorization header value (e.g., Bearer eyJ0eXAiOiJKV1...). | [Defaults to `undefined`] |
| **body** | `string` | Create browser session on behalf of end-user. | |

### Return type

[**CreateBrowserSessionResponse**](CreateBrowserSessionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `text/plain`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Create a Single Sign On (SSO) to access the IBKR hosted portal (White Branded). |  -  |
| **400** | Returns a Problem detail instance representing a bad request. |  -  |
| **401** | Returns a Problem detail instance representing an unauthorized request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1SsoSessions

> CreateSessionResponse postGwApiV1SsoSessions(authorization, body)

Create A New SSO Session On Behalf Of An End-user.

&lt;br&gt;**Scope**: &#x60;sso-sessions.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AuthorizationSSOSessionsApi,
} from 'bezant-client';
import type { PostGwApiV1SsoSessionsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AuthorizationSSOSessionsApi();

  const body = {
    // string | Specifies the authorization header value (e.g., Bearer eyJ0eXAiOiJKV1...).
    authorization: Bearer eyJ0eXAiOiJKV1...,
    // string | Create session on behalf of end-user.
    body: body_example,
  } satisfies PostGwApiV1SsoSessionsRequest;

  try {
    const data = await api.postGwApiV1SsoSessions(body);
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
| **authorization** | `string` | Specifies the authorization header value (e.g., Bearer eyJ0eXAiOiJKV1...). | [Defaults to `undefined`] |
| **body** | `string` | Create session on behalf of end-user. | |

### Return type

[**CreateSessionResponse**](CreateSessionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `text/plain`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns a JSON object containing a reference to the newly created SSO session. |  -  |
| **400** | Returns a Problem detail instance representing a bad request. |  -  |
| **401** | Returns a Problem detail instance representing an unauthorized request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

