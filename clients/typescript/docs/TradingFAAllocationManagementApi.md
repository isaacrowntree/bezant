# TradingFAAllocationManagementApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**createAllocationGroup**](TradingFAAllocationManagementApi.md#createallocationgroupoperation) | **POST** /iserver/account/allocation/group | Add Allocation Group |
| [**deleteAllocationGroup**](TradingFAAllocationManagementApi.md#deleteallocationgroupoperation) | **POST** /iserver/account/allocation/group/delete | Delete An Allocation Group |
| [**getAccountsInModel**](TradingFAAllocationManagementApi.md#getaccountsinmodeloperation) | **POST** /fa/model/accounts-details | Get Models Accounts |
| [**getAllModelPositions**](TradingFAAllocationManagementApi.md#getallmodelpositionsoperation) | **POST** /fa/model/positions | Request Model Positions |
| [**getAllmodels**](TradingFAAllocationManagementApi.md#getallmodels) | **POST** /fa/model/list | Request All Models |
| [**getAllocatableSubaccounts**](TradingFAAllocationManagementApi.md#getallocatablesubaccounts) | **GET** /iserver/account/allocation/accounts | List Allocatable Subaccounts |
| [**getAllocationGroups**](TradingFAAllocationManagementApi.md#getallocationgroups) | **GET** /iserver/account/allocation/group | List All Allocation Groups |
| [**getAllocationPresets**](TradingFAAllocationManagementApi.md#getallocationpresets) | **GET** /iserver/account/allocation/presets | Retrieve Allocation Presets |
| [**getInvestedAccountsInModel**](TradingFAAllocationManagementApi.md#getinvestedaccountsinmodeloperation) | **POST** /fa/model/invest-divest-positions | Summary Of Accounts Invested In The Model |
| [**getModelPresets**](TradingFAAllocationManagementApi.md#getmodelpresetsoperation) | **POST** /fa/fa-preset/get | Get Model Preset |
| [**getModelSummarySingle**](TradingFAAllocationManagementApi.md#getmodelsummarysingleoperation) | **POST** /fa/model/summary | Request Model Summary |
| [**getSingleAllocationGroup**](TradingFAAllocationManagementApi.md#getsingleallocationgroup) | **POST** /iserver/account/allocation/group/single | Retrieve Single Allocation Group |
| [**modifyAllocationGroup**](TradingFAAllocationManagementApi.md#modifyallocationgroupoperation) | **PUT** /iserver/account/allocation/group | Modify Allocation Group |
| [**setAccountinvestmentInModel**](TradingFAAllocationManagementApi.md#setaccountinvestmentinmodeloperation) | **POST** /fa/model/invest-divest | Invest Account Into Model |
| [**setAllocationPreset**](TradingFAAllocationManagementApi.md#setallocationpreset) | **POST** /iserver/account/allocation/presets | Set Allocation Preset |
| [**setModelPresets**](TradingFAAllocationManagementApi.md#setmodelpresets) | **POST** /fa/fa-preset/save | Set Model Preset |
| [**setModelTargetPositions**](TradingFAAllocationManagementApi.md#setmodeltargetpositionsoperation) | **POST** /fa/model/save | Set Model Allocations |
| [**submitModelOrders**](TradingFAAllocationManagementApi.md#submitmodelordersoperation) | **POST** /fa/model/submit-transfers | Submit Transfers |



## createAllocationGroup

> ModifyAllocationGroup200Response createAllocationGroup(createAllocationGroupRequest)

Add Allocation Group

Add a new allocation group. This group can be used to trade in place of the {accountId} for the /iserver/account/{accountId}/orders endpoint.

### Example

```ts
import {
  Configuration,
  TradingFAAllocationManagementApi,
} from 'bezant-client';
import type { CreateAllocationGroupOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFAAllocationManagementApi();

  const body = {
    // CreateAllocationGroupRequest
    createAllocationGroupRequest: {"accounts":[{"name":"U456789"},{"name":"U123456"}],"default_method":"E","name":"Group_1_Equal"},
  } satisfies CreateAllocationGroupOperationRequest;

  try {
    const data = await api.createAllocationGroup(body);
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
| **createAllocationGroupRequest** | [CreateAllocationGroupRequest](CreateAllocationGroupRequest.md) |  | |

### Return type

[**ModifyAllocationGroup200Response**](ModifyAllocationGroup200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns a confirmation that the modification was successful. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## deleteAllocationGroup

> ModifyAllocationGroup200Response deleteAllocationGroup(deleteAllocationGroupRequest)

Delete An Allocation Group

Deletes a previously created allocation group. This endpoint is only supported for Financial Advisors and IBroker Accounts.  

### Example

```ts
import {
  Configuration,
  TradingFAAllocationManagementApi,
} from 'bezant-client';
import type { DeleteAllocationGroupOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFAAllocationManagementApi();

  const body = {
    // DeleteAllocationGroupRequest
    deleteAllocationGroupRequest: ...,
  } satisfies DeleteAllocationGroupOperationRequest;

  try {
    const data = await api.deleteAllocationGroup(body);
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
| **deleteAllocationGroupRequest** | [DeleteAllocationGroupRequest](DeleteAllocationGroupRequest.md) |  | |

### Return type

[**ModifyAllocationGroup200Response**](ModifyAllocationGroup200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns a confirmation that the modification was successful. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getAccountsInModel

> GetAccountsInModel200Response getAccountsInModel(getAccountsInModelRequest)

Get Models Accounts

Request all accounts held within a model.

### Example

```ts
import {
  Configuration,
  TradingFAAllocationManagementApi,
} from 'bezant-client';
import type { GetAccountsInModelOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFAAllocationManagementApi();

  const body = {
    // GetAccountsInModelRequest
    getAccountsInModelRequest: ...,
  } satisfies GetAccountsInModelOperationRequest;

  try {
    const data = await api.getAccountsInModel(body);
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
| **getAccountsInModelRequest** | [GetAccountsInModelRequest](GetAccountsInModelRequest.md) |  | |

### Return type

[**GetAccountsInModel200Response**](GetAccountsInModel200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successfully retrieve account details of in a model. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getAllModelPositions

> ModelPositionResponse getAllModelPositions(getAllModelPositionsRequest)

Request Model Positions

Request all positions held within the model.

### Example

```ts
import {
  Configuration,
  TradingFAAllocationManagementApi,
} from 'bezant-client';
import type { GetAllModelPositionsOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFAAllocationManagementApi();

  const body = {
    // GetAllModelPositionsRequest
    getAllModelPositionsRequest: ...,
  } satisfies GetAllModelPositionsOperationRequest;

  try {
    const data = await api.getAllModelPositions(body);
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
| **getAllModelPositionsRequest** | [GetAllModelPositionsRequest](GetAllModelPositionsRequest.md) |  | |

### Return type

[**ModelPositionResponse**](ModelPositionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successfully retrieve position details |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getAllmodels

> ModelListResponse getAllmodels(getModelPresetsRequest)

Request All Models

Retrieve summaries for all models under the advisor account.

### Example

```ts
import {
  Configuration,
  TradingFAAllocationManagementApi,
} from 'bezant-client';
import type { GetAllmodelsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFAAllocationManagementApi();

  const body = {
    // GetModelPresetsRequest
    getModelPresetsRequest: ...,
  } satisfies GetAllmodelsRequest;

  try {
    const data = await api.getAllmodels(body);
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
| **getModelPresetsRequest** | [GetModelPresetsRequest](GetModelPresetsRequest.md) |  | |

### Return type

[**ModelListResponse**](ModelListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successfully retrieve the model list |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getAllocatableSubaccounts

> SubAccounts getAllocatableSubaccounts()

List Allocatable Subaccounts

Retrieves a list of all sub-accounts and returns their net liquidity and available equity for advisors to make decisions on what accounts should be allocated and how. This endpoint is only supported for Financial Advisors and IBroker Accounts.  

### Example

```ts
import {
  Configuration,
  TradingFAAllocationManagementApi,
} from 'bezant-client';
import type { GetAllocatableSubaccountsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFAAllocationManagementApi();

  try {
    const data = await api.getAllocatableSubaccounts();
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

[**SubAccounts**](SubAccounts.md)

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


## getAllocationGroups

> AllocationGroups getAllocationGroups()

List All Allocation Groups

Retrieves a list of all of the advisor\&#39;s allocation groups. This describes the name of the allocation group, number of subaccounts within the group, and the method in use for the group. This endpoint is only supported for Financial Advisors and IBroker Accounts.  

### Example

```ts
import {
  Configuration,
  TradingFAAllocationManagementApi,
} from 'bezant-client';
import type { GetAllocationGroupsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFAAllocationManagementApi();

  try {
    const data = await api.getAllocationGroups();
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

[**AllocationGroups**](AllocationGroups.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns the \&quot;data\&quot; array, which contains all allocation groups under the advisor account. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getAllocationPresets

> Presets getAllocationPresets()

Retrieve Allocation Presets

Retrieve the preset behavior for allocation groups for specific events. This endpoint is only supported for Financial Advisors and IBroker Accounts.  

### Example

```ts
import {
  Configuration,
  TradingFAAllocationManagementApi,
} from 'bezant-client';
import type { GetAllocationPresetsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFAAllocationManagementApi();

  try {
    const data = await api.getAllocationPresets();
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

[**Presets**](Presets.md)

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


## getInvestedAccountsInModel

> GetInvestedAccountsSummary getInvestedAccountsInModel(getInvestedAccountsInModelRequest)

Summary Of Accounts Invested In The Model

Request the list of all accounts already invested in the provided model and a summary of their investment.

### Example

```ts
import {
  Configuration,
  TradingFAAllocationManagementApi,
} from 'bezant-client';
import type { GetInvestedAccountsInModelOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFAAllocationManagementApi();

  const body = {
    // GetInvestedAccountsInModelRequest
    getInvestedAccountsInModelRequest: ...,
  } satisfies GetInvestedAccountsInModelOperationRequest;

  try {
    const data = await api.getInvestedAccountsInModel(body);
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
| **getInvestedAccountsInModelRequest** | [GetInvestedAccountsInModelRequest](GetInvestedAccountsInModelRequest.md) |  | |

### Return type

[**GetInvestedAccountsSummary**](GetInvestedAccountsSummary.md)

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


## getModelPresets

> ModelPresetsResponse getModelPresets(getModelPresetsRequest)

Get Model Preset

Get the preset behavior for model rebalancing.

### Example

```ts
import {
  Configuration,
  TradingFAAllocationManagementApi,
} from 'bezant-client';
import type { GetModelPresetsOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFAAllocationManagementApi();

  const body = {
    // GetModelPresetsRequest
    getModelPresetsRequest: ...,
  } satisfies GetModelPresetsOperationRequest;

  try {
    const data = await api.getModelPresets(body);
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
| **getModelPresetsRequest** | [GetModelPresetsRequest](GetModelPresetsRequest.md) |  | |

### Return type

[**ModelPresetsResponse**](ModelPresetsResponse.md)

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


## getModelSummarySingle

> ModelSummaryResponse getModelSummarySingle(getModelSummarySingleRequest)

Request Model Summary

Request a summary for a single model.

### Example

```ts
import {
  Configuration,
  TradingFAAllocationManagementApi,
} from 'bezant-client';
import type { GetModelSummarySingleOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFAAllocationManagementApi();

  const body = {
    // GetModelSummarySingleRequest
    getModelSummarySingleRequest: ...,
  } satisfies GetModelSummarySingleOperationRequest;

  try {
    const data = await api.getModelSummarySingle(body);
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
| **getModelSummarySingleRequest** | [GetModelSummarySingleRequest](GetModelSummarySingleRequest.md) |  | |

### Return type

[**ModelSummaryResponse**](ModelSummaryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successfully retrieve model summary. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getSingleAllocationGroup

> AllocationGroup getSingleAllocationGroup(deleteAllocationGroupRequest)

Retrieve Single Allocation Group

Retrieves the configuration of a single account group.  This describes the name of the allocation group, the specific accounts contained in the group, and the allocation method in use along with any relevant quantities. This endpoint is only supported for Financial Advisors and IBroker Accounts.  

### Example

```ts
import {
  Configuration,
  TradingFAAllocationManagementApi,
} from 'bezant-client';
import type { GetSingleAllocationGroupRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFAAllocationManagementApi();

  const body = {
    // DeleteAllocationGroupRequest
    deleteAllocationGroupRequest: ...,
  } satisfies GetSingleAllocationGroupRequest;

  try {
    const data = await api.getSingleAllocationGroup(body);
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
| **deleteAllocationGroupRequest** | [DeleteAllocationGroupRequest](DeleteAllocationGroupRequest.md) |  | |

### Return type

[**AllocationGroup**](AllocationGroup.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns details of the allocation group. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## modifyAllocationGroup

> ModifyAllocationGroup200Response modifyAllocationGroup(modifyAllocationGroupRequest)

Modify Allocation Group

Modify an existing allocation group.

### Example

```ts
import {
  Configuration,
  TradingFAAllocationManagementApi,
} from 'bezant-client';
import type { ModifyAllocationGroupOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFAAllocationManagementApi();

  const body = {
    // ModifyAllocationGroupRequest
    modifyAllocationGroupRequest: {"accounts":[{"name":"U456789"},{"name":"U123456"}],"default_method":"E","name":"Group_1_Equal","prev_name":"Group_0_Equal"},
  } satisfies ModifyAllocationGroupOperationRequest;

  try {
    const data = await api.modifyAllocationGroup(body);
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
| **modifyAllocationGroupRequest** | [ModifyAllocationGroupRequest](ModifyAllocationGroupRequest.md) |  | |

### Return type

[**ModifyAllocationGroup200Response**](ModifyAllocationGroup200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns a confirmation that the modification was successful. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## setAccountinvestmentInModel

> SetAccountinvestmentInModel200Response setAccountinvestmentInModel(setAccountinvestmentInModelRequest)

Invest Account Into Model

Assign an account and the amount of cash to allocate into a model.

### Example

```ts
import {
  Configuration,
  TradingFAAllocationManagementApi,
} from 'bezant-client';
import type { SetAccountinvestmentInModelOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFAAllocationManagementApi();

  const body = {
    // SetAccountinvestmentInModelRequest
    setAccountinvestmentInModelRequest: ...,
  } satisfies SetAccountinvestmentInModelOperationRequest;

  try {
    const data = await api.setAccountinvestmentInModel(body);
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
| **setAccountinvestmentInModelRequest** | [SetAccountinvestmentInModelRequest](SetAccountinvestmentInModelRequest.md) |  | |

### Return type

[**SetAccountinvestmentInModel200Response**](SetAccountinvestmentInModel200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successfully submitted model investment. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## setAllocationPreset

> SetAllocationPreset200Response setAllocationPreset(presets)

Set Allocation Preset

Set the preset behavior for new allocation groups for specific events.

### Example

```ts
import {
  Configuration,
  TradingFAAllocationManagementApi,
} from 'bezant-client';
import type { SetAllocationPresetRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFAAllocationManagementApi();

  const body = {
    // Presets
    presets: ...,
  } satisfies SetAllocationPresetRequest;

  try {
    const data = await api.setAllocationPreset(body);
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
| **presets** | [Presets](Presets.md) |  | |

### Return type

[**SetAllocationPreset200Response**](SetAllocationPreset200Response.md)

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


## setModelPresets

> SetModelPresets200Response setModelPresets(modelPresetsResponse)

Set Model Preset

Set the preset behavior for models.

### Example

```ts
import {
  Configuration,
  TradingFAAllocationManagementApi,
} from 'bezant-client';
import type { SetModelPresetsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFAAllocationManagementApi();

  const body = {
    // ModelPresetsResponse
    modelPresetsResponse: ...,
  } satisfies SetModelPresetsRequest;

  try {
    const data = await api.setModelPresets(body);
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
| **modelPresetsResponse** | [ModelPresetsResponse](ModelPresetsResponse.md) |  | |

### Return type

[**SetModelPresets200Response**](SetModelPresets200Response.md)

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


## setModelTargetPositions

> SetModelTargetPositions200Response setModelTargetPositions(setModelTargetPositionsRequest)

Set Model Allocations

Create or Modify a model\&#39;s target positions.

### Example

```ts
import {
  Configuration,
  TradingFAAllocationManagementApi,
} from 'bezant-client';
import type { SetModelTargetPositionsOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFAAllocationManagementApi();

  const body = {
    // SetModelTargetPositionsRequest
    setModelTargetPositionsRequest: ...,
  } satisfies SetModelTargetPositionsOperationRequest;

  try {
    const data = await api.setModelTargetPositions(body);
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
| **setModelTargetPositionsRequest** | [SetModelTargetPositionsRequest](SetModelTargetPositionsRequest.md) |  | |

### Return type

[**SetModelTargetPositions200Response**](SetModelTargetPositions200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successfully saved a model |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## submitModelOrders

> SubmitModelOrders200Response submitModelOrders(submitModelOrdersRequest)

Submit Transfers

Submit all pending orders to the models. This is similar to the Model page\&#39;s Submit All Orders selection.

### Example

```ts
import {
  Configuration,
  TradingFAAllocationManagementApi,
} from 'bezant-client';
import type { SubmitModelOrdersOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingFAAllocationManagementApi();

  const body = {
    // SubmitModelOrdersRequest
    submitModelOrdersRequest: ...,
  } satisfies SubmitModelOrdersOperationRequest;

  try {
    const data = await api.submitModelOrders(body);
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
| **submitModelOrdersRequest** | [SubmitModelOrdersRequest](SubmitModelOrdersRequest.md) |  | |

### Return type

[**SubmitModelOrders200Response**](SubmitModelOrders200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successfully transmit all orders. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

