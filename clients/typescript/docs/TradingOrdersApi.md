# TradingOrdersApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**ackServerPrompt**](TradingOrdersApi.md#ackserverpromptoperation) | **POST** /iserver/notification | Dismiss Server Prompt |
| [**cancelOpenOrder**](TradingOrdersApi.md#cancelopenorder) | **DELETE** /iserver/account/{accountId}/order/{orderId} | Cancel An Open Order |
| [**confirmOrderReply**](TradingOrdersApi.md#confirmorderreplyoperation) | **POST** /iserver/reply/{replyId} | Confirm Order Reply Message |
| [**getOpenOrders**](TradingOrdersApi.md#getopenorders) | **GET** /iserver/account/orders | List Open Orders |
| [**getOrderStatus**](TradingOrdersApi.md#getorderstatus) | **GET** /iserver/account/order/status/{orderId} | Status Of A Single Order |
| [**getTradeHistory**](TradingOrdersApi.md#gettradehistory) | **GET** /iserver/account/trades | Trade History |
| [**modifyOpenOrder**](TradingOrdersApi.md#modifyopenorder) | **POST** /iserver/account/{accountId}/order/{orderId} | Modify Open Order |
| [**previewMarginImpact**](TradingOrdersApi.md#previewmarginimpact) | **POST** /iserver/account/{accountId}/orders/whatif | New Order Preview |
| [**resetOrderSuppression**](TradingOrdersApi.md#resetordersuppression) | **POST** /iserver/questions/suppress/reset | Reset Order Reply Message Suppression |
| [**submitNewOrder**](TradingOrdersApi.md#submitneworder) | **POST** /iserver/account/{accountId}/orders | Submit New Order |
| [**suppressOrderReplies**](TradingOrdersApi.md#suppressorderrepliesoperation) | **POST** /iserver/questions/suppress | Suppress Order Reply Messages |



## ackServerPrompt

> string ackServerPrompt(ackServerPromptRequest)

Dismiss Server Prompt

Respond to a server prompt received via ntf websocket message.

### Example

```ts
import {
  Configuration,
  TradingOrdersApi,
} from 'bezant-client';
import type { AckServerPromptOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingOrdersApi();

  const body = {
    // AckServerPromptRequest
    ackServerPromptRequest: ...,
  } satisfies AckServerPromptOperationRequest;

  try {
    const data = await api.ackServerPrompt(body);
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
| **ackServerPromptRequest** | [AckServerPromptRequest](AckServerPromptRequest.md) |  | |

### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Status of submitted prompt |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## cancelOpenOrder

> CancelOpenOrder200Response cancelOpenOrder(accountId, orderId, extOperator, manualIndicator, manualCancelTime)

Cancel An Open Order

Cancel an existing, unfilled order.

### Example

```ts
import {
  Configuration,
  TradingOrdersApi,
} from 'bezant-client';
import type { CancelOpenOrderRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingOrdersApi();

  const body = {
    // string | The account to which the order will clear.
    accountId: DU123456,
    // string | The IB-assigned order ID of the desired order ticket.
    orderId: 1799796559,
    // string | ExtOperator is used to identify external operator. (optional)
    extOperator: extOperator_example,
    // boolean | For all orders for US Futures products, clients must submit this flag to indicate whether the order was originated manually (by a natural person) or automatically (by an automated trading system transmitting orders without human intervention). Submit a True value to indicate a manually originated order, and submit a False value to indicate an automated order. Orders for USFUT products that do not include this field will be rejected. (optional)
    manualIndicator: true,
    // number | Time of manual cancel. (optional)
    manualCancelTime: 1799796559,
  } satisfies CancelOpenOrderRequest;

  try {
    const data = await api.cancelOpenOrder(body);
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
| **accountId** | `string` | The account to which the order will clear. | [Defaults to `undefined`] |
| **orderId** | `string` | The IB-assigned order ID of the desired order ticket. | [Defaults to `undefined`] |
| **extOperator** | `string` | ExtOperator is used to identify external operator. | [Optional] [Defaults to `undefined`] |
| **manualIndicator** | `boolean` | For all orders for US Futures products, clients must submit this flag to indicate whether the order was originated manually (by a natural person) or automatically (by an automated trading system transmitting orders without human intervention). Submit a True value to indicate a manually originated order, and submit a False value to indicate an automated order. Orders for USFUT products that do not include this field will be rejected. | [Optional] [Defaults to `undefined`] |
| **manualCancelTime** | `number` | Time of manual cancel. | [Optional] [Defaults to `undefined`] |

### Return type

[**CancelOpenOrder200Response**](CancelOpenOrder200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Status of submission |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## confirmOrderReply

> ConfirmOrderReply200Response confirmOrderReply(replyId, confirmOrderReplyRequest)

Confirm Order Reply Message

Confirm an order reply message and continue with submission of order ticket.

### Example

```ts
import {
  Configuration,
  TradingOrdersApi,
} from 'bezant-client';
import type { ConfirmOrderReplyOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingOrdersApi();

  const body = {
    // string | The UUID of the reply message to be confirmed, obtained from an order submission response. This is delivered from the POST /iserver/account/{accountId}/orders endpoint when triggering certain warning messages.  
    replyId: 99097238-9824-4830-84ef-46979aa22593,
    // ConfirmOrderReplyRequest
    confirmOrderReplyRequest: ...,
  } satisfies ConfirmOrderReplyOperationRequest;

  try {
    const data = await api.confirmOrderReply(body);
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
| **replyId** | `string` | The UUID of the reply message to be confirmed, obtained from an order submission response. This is delivered from the POST /iserver/account/{accountId}/orders endpoint when triggering certain warning messages.   | [Defaults to `undefined`] |
| **confirmOrderReplyRequest** | [ConfirmOrderReplyRequest](ConfirmOrderReplyRequest.md) |  | |

### Return type

[**ConfirmOrderReply200Response**](ConfirmOrderReply200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Status of reply |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getOpenOrders

> LiveOrdersResponse getOpenOrders(filters, force)

List Open Orders

Returns open orders and filled or cancelled orders submitted during the current brokerage session.

### Example

```ts
import {
  Configuration,
  TradingOrdersApi,
} from 'bezant-client';
import type { GetOpenOrdersRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingOrdersApi();

  const body = {
    // 'inactive' | 'pending_submit' | 'pre_submitted' | 'submitted' | 'filled' | 'pending_cancel' | 'cancelled' | 'warn_state' | 'sort_by_time' | Filter results using a comma-separated list of Order Status values. Also accepts a value to sort results by time. (optional)
    filters: filled,
    // boolean | Instructs IB to clear cache of orders and obtain updated view from brokerage backend. Response will be an empty array. (optional)
    force: true,
  } satisfies GetOpenOrdersRequest;

  try {
    const data = await api.getOpenOrders(body);
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
| **filters** | `inactive`, `pending_submit`, `pre_submitted`, `submitted`, `filled`, `pending_cancel`, `cancelled`, `warn_state`, `sort_by_time` | Filter results using a comma-separated list of Order Status values. Also accepts a value to sort results by time. | [Optional] [Defaults to `undefined`] [Enum: inactive, pending_submit, pre_submitted, submitted, filled, pending_cancel, cancelled, warn_state, sort_by_time] |
| **force** | `boolean` | Instructs IB to clear cache of orders and obtain updated view from brokerage backend. Response will be an empty array. | [Optional] [Defaults to `undefined`] |

### Return type

[**LiveOrdersResponse**](LiveOrdersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Orders for a specific account |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getOrderStatus

> OrderStatus getOrderStatus(orderId)

Status Of A Single Order

Retrieve the status of a single order. Only displays orders from the current brokerage session. If orders executed on a previous day or session, queries will 503 error. 

### Example

```ts
import {
  Configuration,
  TradingOrdersApi,
} from 'bezant-client';
import type { GetOrderStatusRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingOrdersApi();

  const body = {
    // number
    orderId: 1799796559,
  } satisfies GetOrderStatusRequest;

  try {
    const data = await api.getOrderStatus(body);
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
| **orderId** | `number` |  | [Defaults to `undefined`] |

### Return type

[**OrderStatus**](OrderStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | order status |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getTradeHistory

> Array&lt;TradesResponseInner&gt; getTradeHistory(days)

Trade History

Retrieve a list of trades, up to a maximum of 7 days prior.

### Example

```ts
import {
  Configuration,
  TradingOrdersApi,
} from 'bezant-client';
import type { GetTradeHistoryRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingOrdersApi();

  const body = {
    // number | The number of prior days prior to include in response, up to a maximum of 7. If omitted, only the current day\'s executions will be returned. (optional)
    days: 3,
  } satisfies GetTradeHistoryRequest;

  try {
    const data = await api.getTradeHistory(body);
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
| **days** | `number` | The number of prior days prior to include in response, up to a maximum of 7. If omitted, only the current day\&#39;s executions will be returned. | [Optional] [Defaults to `undefined`] |

### Return type

[**Array&lt;TradesResponseInner&gt;**](TradesResponseInner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | trades |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## modifyOpenOrder

> ModifyOpenOrder200Response modifyOpenOrder(accountId, orderId, singleOrderSubmissionRequest)

Modify Open Order

Modify an existing, unfilled order.

### Example

```ts
import {
  Configuration,
  TradingOrdersApi,
} from 'bezant-client';
import type { ModifyOpenOrderRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingOrdersApi();

  const body = {
    // string | The account to which the order will clear.
    accountId: DU123456,
    // number | The IB-assigned order ID of the desired order ticket.
    orderId: 1799796559,
    // SingleOrderSubmissionRequest
    singleOrderSubmissionRequest: ...,
  } satisfies ModifyOpenOrderRequest;

  try {
    const data = await api.modifyOpenOrder(body);
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
| **accountId** | `string` | The account to which the order will clear. | [Defaults to `undefined`] |
| **orderId** | `number` | The IB-assigned order ID of the desired order ticket. | [Defaults to `undefined`] |
| **singleOrderSubmissionRequest** | [SingleOrderSubmissionRequest](SingleOrderSubmissionRequest.md) |  | |

### Return type

[**ModifyOpenOrder200Response**](ModifyOpenOrder200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Status of submission |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## previewMarginImpact

> OrderPreview previewMarginImpact(accountId, ordersSubmissionRequest)

New Order Preview

Preview the projected effects of an order ticket or bracket of orders, including cost and changes to margin and account equity.

### Example

```ts
import {
  Configuration,
  TradingOrdersApi,
} from 'bezant-client';
import type { PreviewMarginImpactRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingOrdersApi();

  const body = {
    // string | The account to which the order will clear.
    accountId: DU123456,
    // OrdersSubmissionRequest
    ordersSubmissionRequest: ...,
  } satisfies PreviewMarginImpactRequest;

  try {
    const data = await api.previewMarginImpact(body);
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
| **accountId** | `string` | The account to which the order will clear. | [Defaults to `undefined`] |
| **ordersSubmissionRequest** | [OrdersSubmissionRequest](OrdersSubmissionRequest.md) |  | |

### Return type

[**OrderPreview**](OrderPreview.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Order Preview |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## resetOrderSuppression

> ResetOrderSuppression200Response resetOrderSuppression()

Reset Order Reply Message Suppression

Removes suppression of all order reply messages that were previously suppressed in the current brokerage session.

### Example

```ts
import {
  Configuration,
  TradingOrdersApi,
} from 'bezant-client';
import type { ResetOrderSuppressionRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingOrdersApi();

  try {
    const data = await api.resetOrderSuppression();
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

[**ResetOrderSuppression200Response**](ResetOrderSuppression200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Request\&#39;s status |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## submitNewOrder

> SubmitNewOrder200Response submitNewOrder(accountId, ordersSubmissionRequest)

Submit New Order

Submit a new order(s) ticket, bracket, or OCA group.

### Example

```ts
import {
  Configuration,
  TradingOrdersApi,
} from 'bezant-client';
import type { SubmitNewOrderRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingOrdersApi();

  const body = {
    // string | The account to which the order will clear.
    accountId: DU123456,
    // OrdersSubmissionRequest
    ordersSubmissionRequest: ...,
  } satisfies SubmitNewOrderRequest;

  try {
    const data = await api.submitNewOrder(body);
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
| **accountId** | `string` | The account to which the order will clear. | [Defaults to `undefined`] |
| **ordersSubmissionRequest** | [OrdersSubmissionRequest](OrdersSubmissionRequest.md) |  | |

### Return type

[**SubmitNewOrder200Response**](SubmitNewOrder200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Order submission response (success, error, reply is required or order reject) |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## suppressOrderReplies

> SuppressOrderReplies200Response suppressOrderReplies(suppressOrderRepliesRequest)

Suppress Order Reply Messages

Suppress the specified order reply messages for the duration of the brokerage session.

### Example

```ts
import {
  Configuration,
  TradingOrdersApi,
} from 'bezant-client';
import type { SuppressOrderRepliesOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingOrdersApi();

  const body = {
    // SuppressOrderRepliesRequest
    suppressOrderRepliesRequest: ...,
  } satisfies SuppressOrderRepliesOperationRequest;

  try {
    const data = await api.suppressOrderReplies(body);
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
| **suppressOrderRepliesRequest** | [SuppressOrderRepliesRequest](SuppressOrderRepliesRequest.md) |  | |

### Return type

[**SuppressOrderReplies200Response**](SuppressOrderReplies200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Request\&#39;s status |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

