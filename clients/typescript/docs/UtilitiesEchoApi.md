# UtilitiesEchoApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**getGwApiV1EchoHttps**](UtilitiesEchoApi.md#getgwapiv1echohttps) | **GET** /gw/api/v1/echo/https | Echo A Request With HTTPS Security Policy Back After Validation. |
| [**postGwApiV1EchoSignedJwt**](UtilitiesEchoApi.md#postgwapiv1echosignedjwt) | **POST** /gw/api/v1/echo/signed-jwt | Echo A Request With Signed JWT Security Policy Back After Validation. |



## getGwApiV1EchoHttps

> EchoResponse getGwApiV1EchoHttps()

Echo A Request With HTTPS Security Policy Back After Validation.

&lt;br&gt;**Scope**: &#x60;echo.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  UtilitiesEchoApi,
} from 'bezant-client';
import type { GetGwApiV1EchoHttpsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new UtilitiesEchoApi();

  try {
    const data = await api.getGwApiV1EchoHttps();
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

[**EchoResponse**](EchoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns a JSON object containing the request parameters. |  -  |
| **401** | Returns a Problem detail instance representing an unauthorized request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1EchoSignedJwt

> EchoResponse postGwApiV1EchoSignedJwt(body)

Echo A Request With Signed JWT Security Policy Back After Validation.

&lt;br&gt;**Scope**: &#x60;echo.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  UtilitiesEchoApi,
} from 'bezant-client';
import type { PostGwApiV1EchoSignedJwtRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new UtilitiesEchoApi();

  const body = {
    // string | Create a Signed JWT echo request.
    body: body_example,
  } satisfies PostGwApiV1EchoSignedJwtRequest;

  try {
    const data = await api.postGwApiV1EchoSignedJwt(body);
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
| **body** | `string` | Create a Signed JWT echo request. | |

### Return type

[**EchoResponse**](EchoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `text/plain`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns a JSON object containing the request parameters. |  -  |
| **401** | Returns a Problem detail instance representing an unauthorized request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

