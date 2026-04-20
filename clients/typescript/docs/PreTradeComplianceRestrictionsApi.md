# PreTradeComplianceRestrictionsApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**applyCSV**](PreTradeComplianceRestrictionsApi.md#applycsv) | **POST** /gw/api/v1/restrictions | Apply PTC CSV |
| [**verifyCSV**](PreTradeComplianceRestrictionsApi.md#verifycsv) | **POST** /gw/api/v1/restrictions/verify | Verify PTC CSV |



## applyCSV

> CSVResponse applyCSV(authorization, body)

Apply PTC CSV

Applies verified CSV changes. Requires both Bearer token (header) and signed JWT (body). CSV must be verified via /csv/v2/verify first. JWT validity: 1 minute.&lt;br&gt;&lt;br&gt;**Scope**: &#x60;restrictions.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  PreTradeComplianceRestrictionsApi,
} from 'bezant-client';
import type { ApplyCSVRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new PreTradeComplianceRestrictionsApi();

  const body = {
    // string | OAuth 2.0 Bearer token. Format: Bearer {access_token}. Token contains accountId claim. Validity: ~50 minutes.
    authorization: Bearer eyJ0eXAiOiJhdCtKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6IjFkOTEx...,
    // string | Signed JWT token (format: header.payload.signature). Generate using: oauth2_cli sign -C userName={user} -C requestId={id} -C payload={csv} -C iss={user} -k {key}
    body: body_example,
  } satisfies ApplyCSVRequest;

  try {
    const data = await api.applyCSV(body);
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
| **authorization** | `string` | OAuth 2.0 Bearer token. Format: Bearer {access_token}. Token contains accountId claim. Validity: ~50 minutes. | [Defaults to `undefined`] |
| **body** | `string` | Signed JWT token (format: header.payload.signature). Generate using: oauth2_cli sign -C userName&#x3D;{user} -C requestId&#x3D;{id} -C payload&#x3D;{csv} -C iss&#x3D;{user} -k {key} | |

### Return type

[**CSVResponse**](CSVResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `text/plain`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | CSV application result |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **403** | Insufficient permissions or JWT validation failed |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## verifyCSV

> CSVResponse verifyCSV(authorization, verifyRequest)

Verify PTC CSV

Validates CSV structure, restriction names, and rule parameters without applying changes. Must be called before /csv/v2/apply with the same requestId.&lt;br&gt;&lt;br&gt;**Scope**: &#x60;restrictions.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  PreTradeComplianceRestrictionsApi,
} from 'bezant-client';
import type { VerifyCSVRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new PreTradeComplianceRestrictionsApi();

  const body = {
    // string | OAuth 2.0 Bearer token. Format: Bearer {access_token}. Token contains accountId claim. Validity: ~50 minutes.
    authorization: Bearer eyJ0eXAiOiJhdCtKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6IjFkOTEx...,
    // VerifyRequest | JSON with userName, requestId, and Base64-encoded CSV payload
    verifyRequest: {"payload":"UkVTVFJfQURELCBUZXN0UmVzdHJpY3Rpb24sIFJVTEVfQkVHSU4sIHJ1bGVfdHlwZT1DTE9TSU5HT05MWSwgdGlmPUdUQywgUlVMRV9FTkQ=","requestId":127,"userName":"hatty597"},
  } satisfies VerifyCSVRequest;

  try {
    const data = await api.verifyCSV(body);
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
| **authorization** | `string` | OAuth 2.0 Bearer token. Format: Bearer {access_token}. Token contains accountId claim. Validity: ~50 minutes. | [Defaults to `undefined`] |
| **verifyRequest** | [VerifyRequest](VerifyRequest.md) | JSON with userName, requestId, and Base64-encoded CSV payload | |

### Return type

[**CSVResponse**](CSVResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | CSV verification result |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **403** | Insufficient permissions or JWT validation failed |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

