# AuthorizationTokenApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**generateToken**](AuthorizationTokenApi.md#generatetoken) | **POST** /oauth2/api/v1/token | Create Access Token |



## generateToken

> TokenResponse generateToken(clientAssertion, clientAssertionType, clientAuthenticationMethod, clientId, clientSecret)

Create Access Token

Generate OAuth 2.0 access tokens based on request parameters.

### Example

```ts
import {
  Configuration,
  AuthorizationTokenApi,
} from 'bezant-client';
import type { GenerateTokenRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AuthorizationTokenApi();

  const body = {
    // string (optional)
    clientAssertion: clientAssertion_example,
    // string (optional)
    clientAssertionType: clientAssertionType_example,
    // string (optional)
    clientAuthenticationMethod: clientAuthenticationMethod_example,
    // string (optional)
    clientId: clientId_example,
    // string (optional)
    clientSecret: clientSecret_example,
  } satisfies GenerateTokenRequest;

  try {
    const data = await api.generateToken(body);
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
| **clientAssertion** | `string` |  | [Optional] [Defaults to `undefined`] |
| **clientAssertionType** | `string` |  | [Optional] [Defaults to `undefined`] |
| **clientAuthenticationMethod** | `private_key_jwt`, `client_secret_basic`, `client_secret_post` |  | [Optional] [Defaults to `undefined`] [Enum: private_key_jwt, client_secret_basic, client_secret_post] |
| **clientId** | `string` |  | [Optional] [Defaults to `undefined`] |
| **clientSecret** | `string` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**TokenResponse**](TokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/x-www-form-urlencoded`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns a token response. |  -  |
| **400** | Returns a [Problem detail](https://datatracker.ietf.org/doc/html/rfc9457) instance representing a bad request. |  -  |
| **500** | Returns a [Problem detail](https://datatracker.ietf.org/doc/html/rfc9457) instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

