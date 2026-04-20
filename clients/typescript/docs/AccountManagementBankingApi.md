# AccountManagementBankingApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**getGwApiV1ClientInstructionsClientinstructionid**](AccountManagementBankingApi.md#getgwapiv1clientinstructionsclientinstructionid) | **GET** /gw/api/v1/client-instructions/{clientInstructionId} | Get Status For ClientInstructionId |
| [**getGwApiV1InstructionSetsInstructionsetid**](AccountManagementBankingApi.md#getgwapiv1instructionsetsinstructionsetid) | **GET** /gw/api/v1/instruction-sets/{instructionSetId} | Get Status For InstructionSetId |
| [**getGwApiV1InstructionsInstructionid**](AccountManagementBankingApi.md#getgwapiv1instructionsinstructionid) | **GET** /gw/api/v1/instructions/{instructionId} | Get Status For InstructionId |
| [**postGwApiV1BankInstructions**](AccountManagementBankingApi.md#postgwapiv1bankinstructionsoperation) | **POST** /gw/api/v1/bank-instructions | Manage Bank Instructions |
| [**postGwApiV1BankInstructionsBulk**](AccountManagementBankingApi.md#postgwapiv1bankinstructionsbulkoperation) | **POST** /gw/api/v1/bank-instructions:bulk | Creates Multiple Banking Instructions(ach, Delete, Micro-amount, Predefined-destination-instruction) |
| [**postGwApiV1BankInstructionsQuery**](AccountManagementBankingApi.md#postgwapiv1bankinstructionsqueryoperation) | **POST** /gw/api/v1/bank-instructions/query | View Bank Instructions |
| [**postGwApiV1ExternalAssetTransfers**](AccountManagementBankingApi.md#postgwapiv1externalassettransfersoperation) | **POST** /gw/api/v1/external-asset-transfers | Transfer Positions Externally (ACATS, ATON, FOP, DWAC, Complex Asset Transfer) |
| [**postGwApiV1ExternalAssetTransfersBulk**](AccountManagementBankingApi.md#postgwapiv1externalassettransfersbulkoperation) | **POST** /gw/api/v1/external-asset-transfers:bulk | Creates Multiple External Asset Transfers (Fop, DWAC And Complex Asset Transfer) |
| [**postGwApiV1ExternalCashTransfers**](AccountManagementBankingApi.md#postgwapiv1externalcashtransfersoperation) | **POST** /gw/api/v1/external-cash-transfers | Transfer Cash Externally |
| [**postGwApiV1ExternalCashTransfersBulk**](AccountManagementBankingApi.md#postgwapiv1externalcashtransfersbulkoperation) | **POST** /gw/api/v1/external-cash-transfers:bulk | Creates Multiple External Cash Transfers (Deposit And Withdraw Fund) |
| [**postGwApiV1ExternalCashTransfersQuery**](AccountManagementBankingApi.md#postgwapiv1externalcashtransfersqueryoperation) | **POST** /gw/api/v1/external-cash-transfers/query | View Cash Balances |
| [**postGwApiV1InstructionsCancel**](AccountManagementBankingApi.md#postgwapiv1instructionscanceloperation) | **POST** /gw/api/v1/instructions/cancel | Cancel Request |
| [**postGwApiV1InstructionsCancelBulk**](AccountManagementBankingApi.md#postgwapiv1instructionscancelbulkoperation) | **POST** /gw/api/v1/instructions/cancel:bulk | Creates Multiple Cancel Instructions |
| [**postGwApiV1InstructionsQuery**](AccountManagementBankingApi.md#postgwapiv1instructionsqueryoperation) | **POST** /gw/api/v1/instructions/query | Get Transaction History |
| [**postGwApiV1InternalAssetTransfers**](AccountManagementBankingApi.md#postgwapiv1internalassettransfersoperation) | **POST** /gw/api/v1/internal-asset-transfers | Transfer Positions Internally |
| [**postGwApiV1InternalAssetTransfersBulk**](AccountManagementBankingApi.md#postgwapiv1internalassettransfersbulkoperation) | **POST** /gw/api/v1/internal-asset-transfers:bulk | Creates Multiple Internal Asset Transfers Between The Provided Account Id Pairs |
| [**postGwApiV1InternalCashTransfers**](AccountManagementBankingApi.md#postgwapiv1internalcashtransfersoperation) | **POST** /gw/api/v1/internal-cash-transfers | Transfer Cash Internally |
| [**postGwApiV1InternalCashTransfersBulk**](AccountManagementBankingApi.md#postgwapiv1internalcashtransfersbulkoperation) | **POST** /gw/api/v1/internal-cash-transfers:bulk | Creates Multiple Internal Cash Transfers Between The Provided Account Id Pairs |



## getGwApiV1ClientInstructionsClientinstructionid

> GetGwApiV1ClientInstructionsClientinstructionid200Response getGwApiV1ClientInstructionsClientinstructionid(clientInstructionId, clientId)

Get Status For ClientInstructionId

Retrieve status of request by clientInstructionId.&lt;br&gt;&lt;br&gt;**Scope**: &#x60;instructions.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementBankingApi,
} from 'bezant-client';
import type { GetGwApiV1ClientInstructionsClientinstructionidRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementBankingApi();

  const body = {
    // number | The target instruction id.
    clientInstructionId: -1988905739,
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies GetGwApiV1ClientInstructionsClientinstructionidRequest;

  try {
    const data = await api.getGwApiV1ClientInstructionsClientinstructionid(body);
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
| **clientInstructionId** | `number` | The target instruction id. | [Defaults to `undefined`] |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**GetGwApiV1ClientInstructionsClientinstructionid200Response**](GetGwApiV1ClientInstructionsClientinstructionid200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns the status of an instruction. |  -  |
| **404** | Returns a Problem detail instance representing a not found request. |  -  |
| **500** | Unable to process request due to an Internal Error. Please try again later. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getGwApiV1InstructionSetsInstructionsetid

> BulkMultiStatusResponse getGwApiV1InstructionSetsInstructionsetid(instructionSetId, clientId)

Get Status For InstructionSetId

Retrieve status of all requests associated with instructionSetId.&lt;br&gt;&lt;br&gt;**Scope**: &#x60;instructions.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementBankingApi,
} from 'bezant-client';
import type { GetGwApiV1InstructionSetsInstructionsetidRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementBankingApi();

  const body = {
    // number | The target instruction set id.
    instructionSetId: -1988905739,
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies GetGwApiV1InstructionSetsInstructionsetidRequest;

  try {
    const data = await api.getGwApiV1InstructionSetsInstructionsetid(body);
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
| **instructionSetId** | `number` | The target instruction set id. | [Defaults to `undefined`] |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**BulkMultiStatusResponse**](BulkMultiStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns the status multiple instructions. |  -  |
| **404** | Returns a Problem detail instance representing a not found request. |  -  |
| **500** | Unable to process request due to an Internal Error. Please try again later. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getGwApiV1InstructionsInstructionid

> GetGwApiV1ClientInstructionsClientinstructionid200Response getGwApiV1InstructionsInstructionid(instructionId, clientId)

Get Status For InstructionId

Retrieve status of request by instructionId&lt;br&gt;&lt;br&gt;**Scope**: &#x60;instructions.read&#x60;&lt;br&gt;**Security Policy**: &#x60;HTTPS&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementBankingApi,
} from 'bezant-client';
import type { GetGwApiV1InstructionsInstructionidRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementBankingApi();

  const body = {
    // number | The target instruction id.
    instructionId: -1988905739,
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies GetGwApiV1InstructionsInstructionidRequest;

  try {
    const data = await api.getGwApiV1InstructionsInstructionid(body);
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
| **instructionId** | `number` | The target instruction id. | [Defaults to `undefined`] |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**GetGwApiV1ClientInstructionsClientinstructionid200Response**](GetGwApiV1ClientInstructionsClientinstructionid200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns the status of an instruction. |  -  |
| **404** | Returns a Problem detail instance representing a not found request. |  -  |
| **500** | Unable to process request due to an Internal Error. Please try again later. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1BankInstructions

> AsynchronousInstructionResponse postGwApiV1BankInstructions(postGwApiV1BankInstructionsRequest, clientId)

Manage Bank Instructions

Create or delete bank instructions by accountId. Only ACH and EDDA are supported for \&#39;Create\&#39;.&lt;br&gt;&lt;br&gt;**Scope**: &#x60;bank-instructions.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementBankingApi,
} from 'bezant-client';
import type { PostGwApiV1BankInstructionsOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementBankingApi();

  const body = {
    // PostGwApiV1BankInstructionsRequest
    postGwApiV1BankInstructionsRequest: {"instruction":{"accountId":"U223454","achType":"DEBIT_CREDIT","bankInstructionCode":"USACH","bankInstructionName":"TestInstr","clientAccountInfo":{"bankAccountNumber":"101267576983","bankAccountTypeCode":0,"bankName":"JPM Chase","bankRoutingNumber":"202012983"},"clientInstructionId":1012983,"currency":"USD"},"instructionType":"ACH_INSTRUCTION"},
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies PostGwApiV1BankInstructionsOperationRequest;

  try {
    const data = await api.postGwApiV1BankInstructions(body);
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
| **postGwApiV1BankInstructionsRequest** | [PostGwApiV1BankInstructionsRequest](PostGwApiV1BankInstructionsRequest.md) |  | |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**AsynchronousInstructionResponse**](AsynchronousInstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **202** | Accepts request to create a new instruction asynchronously |  -  |
| **400** | Returns a Problem detail instance representing a bad request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **422** | Returns a Problem detail instance representing a business error. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1BankInstructionsBulk

> AsynchronousInstructionSetResponse postGwApiV1BankInstructionsBulk(postGwApiV1BankInstructionsBulkRequest, clientId)

Creates Multiple Banking Instructions(ach, Delete, Micro-amount, Predefined-destination-instruction)

&lt;br&gt;**Scope**: &#x60;bank-instructions.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementBankingApi,
} from 'bezant-client';
import type { PostGwApiV1BankInstructionsBulkOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementBankingApi();

  const body = {
    // PostGwApiV1BankInstructionsBulkRequest | Create Bulk Banking Instructions request body
    postGwApiV1BankInstructionsBulkRequest: {"instructionType":"ACH_INSTRUCTION","instructions":[{"accountId":"U117717","achType":"DEBIT_CREDIT","bankInstructionCode":"USACH","bankInstructionName":"TestInstr","clientAccountInfo":{"bankAccountNumber":"101267576983","bankAccountTypeCode":1,"bankName":"JPM Chase","bankRoutingNumber":"202012983"},"clientInstructionId":7011149,"currency":"USD"},{"accountId":"U117717","achType":"DEBIT_CREDIT","bankInstructionCode":"USACH","bankInstructionName":"TestInstr","clientAccountInfo":{"bankAccountNumber":"101267576983","bankAccountTypeCode":1,"bankName":"JPM Chase","bankRoutingNumber":"202012983"},"clientInstructionId":7011150,"currency":"USD"}]},
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies PostGwApiV1BankInstructionsBulkOperationRequest;

  try {
    const data = await api.postGwApiV1BankInstructionsBulk(body);
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
| **postGwApiV1BankInstructionsBulkRequest** | [PostGwApiV1BankInstructionsBulkRequest](PostGwApiV1BankInstructionsBulkRequest.md) | Create Bulk Banking Instructions request body | |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**AsynchronousInstructionSetResponse**](AsynchronousInstructionSetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **202** | Accepts all instructions in Bulk request to create them asynchronously |  -  |
| **400** | Returns a Problem detail instance representing a bad request. Returned even if only one instruction in the bulk upload has syntactical errors. |  -  |
| **500** | Unable to process request due to an Internal Error. Please try again later. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1BankInstructionsQuery

> PostGwApiV1BankInstructionsQuery201Response postGwApiV1BankInstructionsQuery(postGwApiV1BankInstructionsQueryRequest, clientId)

View Bank Instructions

View active bank instructions for an accountId.&lt;br&gt;&lt;br&gt;**Scope**: &#x60;bank-instructions.read&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementBankingApi,
} from 'bezant-client';
import type { PostGwApiV1BankInstructionsQueryOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementBankingApi();

  const body = {
    // PostGwApiV1BankInstructionsQueryRequest | Create get instruction name request body
    postGwApiV1BankInstructionsQueryRequest: {"instruction":{"accountId":"U46377","bankInstructionMethod":"ACH","clientInstructionId":1012983},"instructionType":"QUERY_BANK_INSTRUCTION"},
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies PostGwApiV1BankInstructionsQueryOperationRequest;

  try {
    const data = await api.postGwApiV1BankInstructionsQuery(body);
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
| **postGwApiV1BankInstructionsQueryRequest** | [PostGwApiV1BankInstructionsQueryRequest](PostGwApiV1BankInstructionsQueryRequest.md) | Create get instruction name request body | |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**PostGwApiV1BankInstructionsQuery201Response**](PostGwApiV1BankInstructionsQuery201Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Instruction successfully created and processed synchronously |  -  |
| **400** | Returns a Problem detail instance representing a bad request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **422** | Returns a Problem detail instance representing a business error. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1ExternalAssetTransfers

> AsynchronousInstructionResponse postGwApiV1ExternalAssetTransfers(postGwApiV1ExternalAssetTransfersRequest, clientId)

Transfer Positions Externally (ACATS, ATON, FOP, DWAC, Complex Asset Transfer)

Initiate request to submit external position transfer. Methods- ACATS, ATON, Basic FOP, FOP, DWAC. More information on transfer methods can be found here - https://www.interactivebrokers.com/campus/trading-lessons/cash-and-position-transfers/&lt;br&gt;&lt;br&gt;**Scope**: &#x60;transfers.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementBankingApi,
} from 'bezant-client';
import type { PostGwApiV1ExternalAssetTransfersOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementBankingApi();

  const body = {
    // PostGwApiV1ExternalAssetTransfersRequest
    postGwApiV1ExternalAssetTransfersRequest: {"instruction":{"accountId":"U399192","clientInstructionId":7013040,"contraBrokerInfo":{"accountType":"ORG","brokerAccountId":"as3456567678578N","brokerName":"JP MORGAN","contactEmail":"a@gmail.com","contactName":"as","contactPhone":"2039126155","country":"United States","depositoryId":"1234"},"direction":"IN","quantity":10,"tradingInstrument":{"conid":12123,"currency":"USD"}},"instructionType":"COMPLEX_ASSET_TRANSFER"},
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies PostGwApiV1ExternalAssetTransfersOperationRequest;

  try {
    const data = await api.postGwApiV1ExternalAssetTransfers(body);
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
| **postGwApiV1ExternalAssetTransfersRequest** | [PostGwApiV1ExternalAssetTransfersRequest](PostGwApiV1ExternalAssetTransfersRequest.md) |  | |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**AsynchronousInstructionResponse**](AsynchronousInstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **202** | Accepts request to create a new instruction asynchronously |  -  |
| **400** | Returns a Problem detail instance representing a bad request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **422** | Returns a Problem detail instance representing a business error. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1ExternalAssetTransfersBulk

> AsynchronousInstructionSetResponse postGwApiV1ExternalAssetTransfersBulk(postGwApiV1ExternalAssetTransfersBulkRequest, clientId)

Creates Multiple External Asset Transfers (Fop, DWAC And Complex Asset Transfer)

&lt;br&gt;**Scope**: &#x60;transfers.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementBankingApi,
} from 'bezant-client';
import type { PostGwApiV1ExternalAssetTransfersBulkOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementBankingApi();

  const body = {
    // PostGwApiV1ExternalAssetTransfersBulkRequest | Create Bulk External Asset transfers (Fop, DWAC and Complex Asset Transfer) request body
    postGwApiV1ExternalAssetTransfersBulkRequest: {"instructionType":"COMPLEX_ASSET_TRANSFER","instructions":[{"accountId":"U399192","clientInstructionId":7013031,"contraBrokerInfo":{"accountType":"ORG","brokerAccountId":"as3456567678578N","brokerName":"JP MORGAN","contactEmail":"a@gmail.com","contactName":"as","contactPhone":"2039126155","country":"United States","depositoryId":"1234"},"direction":"IN","nonDisclosedDetail":{"buyerSellBic":"320043","memberAccountId":"12334","psetBic":"DTCYUS33XXX","reagDeagBic":"CH100164","safeKeepingAccountId":"123456","settleDate":"2018-03-20T09:12:13Z","tradeDate":"2018-03-20T09:12:13Z"},"quantity":10,"tradingInstrument":{"currency":"USD","tradingInstrumentDescription":{"assetType":"STK","securityId":"459200101","securityIdType":"ISIN"}}},{"accountId":"U399192","clientInstructionId":7013032,"contraBrokerInfo":{"accountType":"ORG","brokerAccountId":"as3456567678578N","brokerName":"JP MORGAN","contactEmail":"a@gmail.com","contactName":"as","contactPhone":"2039126155","country":"United States","depositoryId":"1234"},"direction":"IN","quantity":10,"tradingInstrument":{"currency":"USD","tradingInstrumentDescription":{"assetType":"STK","securityId":"459200101","securityIdType":"ISIN"}}},{"accountId":"U399192","clientInstructionId":7013033,"contraBrokerInfo":{"accountType":"ORG","brokerAccountId":"as3456567678578N","brokerName":"JP MORGAN","contactEmail":"a@gmail.com","contactName":"as","contactPhone":"2039126155","country":"United States","depositoryId":"1234"},"direction":"IN","quantity":10,"tradingInstrument":{"conid":12123,"currency":"USD"}}]},
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies PostGwApiV1ExternalAssetTransfersBulkOperationRequest;

  try {
    const data = await api.postGwApiV1ExternalAssetTransfersBulk(body);
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
| **postGwApiV1ExternalAssetTransfersBulkRequest** | [PostGwApiV1ExternalAssetTransfersBulkRequest](PostGwApiV1ExternalAssetTransfersBulkRequest.md) | Create Bulk External Asset transfers (Fop, DWAC and Complex Asset Transfer) request body | |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**AsynchronousInstructionSetResponse**](AsynchronousInstructionSetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **202** | Accepts all instructions in Bulk request to create them asynchronously |  -  |
| **400** | Returns a Problem detail instance representing a bad request. Returned even if only one instruction in the bulk upload has syntactical errors. |  -  |
| **500** | Unable to process request due to an Internal Error. Please try again later. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1ExternalCashTransfers

> AsynchronousInstructionResponse postGwApiV1ExternalCashTransfers(postGwApiV1ExternalCashTransfersRequest, clientId)

Transfer Cash Externally

Initiate request to deposit or withdrawal between IBKR account and bank account. More information on transfer methods can be found here - https://www.interactivebrokers.com/campus/trading-lessons/cash-and-position-transfers&lt;br&gt;&lt;br&gt;**Scope**: &#x60;transfers.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementBankingApi,
} from 'bezant-client';
import type { PostGwApiV1ExternalCashTransfersOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementBankingApi();

  const body = {
    // PostGwApiV1ExternalCashTransfersRequest
    postGwApiV1ExternalCashTransfersRequest: {"instruction":{"accountId":"U46377","amount":100,"bankInstructionMethod":"WIRE","bankInstructionName":"Instruction","clientInstructionId":7013045,"currency":"USD","fromAccountNumber":"U46377","identifier":"indentifier","senderInstitutionName":"Senders Institution name","sendingInstitution":"Sending Institution name","specialInstruction":"U46377"},"instructionType":"DEPOSIT"},
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies PostGwApiV1ExternalCashTransfersOperationRequest;

  try {
    const data = await api.postGwApiV1ExternalCashTransfers(body);
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
| **postGwApiV1ExternalCashTransfersRequest** | [PostGwApiV1ExternalCashTransfersRequest](PostGwApiV1ExternalCashTransfersRequest.md) |  | |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**AsynchronousInstructionResponse**](AsynchronousInstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **202** | Accepts request to create a new instruction asynchronously |  -  |
| **400** | Returns a Problem detail instance representing a bad request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **422** | Returns a Problem detail instance representing a business error. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1ExternalCashTransfersBulk

> AsynchronousInstructionSetResponse postGwApiV1ExternalCashTransfersBulk(postGwApiV1ExternalCashTransfersBulkRequest, clientId)

Creates Multiple External Cash Transfers (Deposit And Withdraw Fund)

&lt;br&gt;**Scope**: &#x60;transfers.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementBankingApi,
} from 'bezant-client';
import type { PostGwApiV1ExternalCashTransfersBulkOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementBankingApi();

  const body = {
    // PostGwApiV1ExternalCashTransfersBulkRequest | Create Bulk External Cash transfers (Deposit and Withdraw fund) request body
    postGwApiV1ExternalCashTransfersBulkRequest: {"instructionType":"DEPOSIT","instructions":[{"accountId":"U46377","amount":100,"bankInstructionMethod":"WIRE","bankInstructionName":"Instruction","clientInstructionId":7013002,"currency":"USD","fromAccountNumber":"U46377","identifier":"indentifier","senderInstitutionName":"Senders Institution name","sendingInstitution":"Sending Institution name","specialInstruction":"U46377"},{"accountId":"U399192","amount":1,"bankInstructionMethod":"WIRE","bankInstructionName":"Instruction","clientInstructionId":7013003,"currency":"USD","fromAccountNumber":"U399192","identifier":"indentifier","iraDepositDetail":{"fromIraType":"TRADITIONAL","iraContributionType":"ROLLOVER","iraTaxYearType":"CURRENT"},"senderInstitutionName":"Senders Institution name","sendingInstitution":"Sending Institution name","specialInstruction":"U399192"},{"accountId":"U46377","amount":100,"bankInstructionMethod":"WIRE","bankInstructionName":"Instruction","clientInstructionId":7013004,"currency":"USD","identifier":"indentifier","isIRA":"false","recurringInstructionDetail":{"frequency":"MONTHLY","instructionName":"Arkansas-Test-Instr","startDate":"2023-10-16"},"senderInstitutionName":"Senders Institution name","sendingInstitution":"Sending Institution name","specialInstruction":"U46377"}]},
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies PostGwApiV1ExternalCashTransfersBulkOperationRequest;

  try {
    const data = await api.postGwApiV1ExternalCashTransfersBulk(body);
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
| **postGwApiV1ExternalCashTransfersBulkRequest** | [PostGwApiV1ExternalCashTransfersBulkRequest](PostGwApiV1ExternalCashTransfersBulkRequest.md) | Create Bulk External Cash transfers (Deposit and Withdraw fund) request body | |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**AsynchronousInstructionSetResponse**](AsynchronousInstructionSetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **202** | Accepts all instructions in Bulk request to create them asynchronously |  -  |
| **400** | Returns a Problem detail instance representing a bad request. Returned even if only one instruction in the bulk upload has syntactical errors. |  -  |
| **500** | Unable to process request due to an Internal Error. Please try again later. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1ExternalCashTransfersQuery

> PostGwApiV1ExternalCashTransfersQuery201Response postGwApiV1ExternalCashTransfersQuery(postGwApiV1ExternalCashTransfersQueryRequest, clientId)

View Cash Balances

View available cash for withdrawal with and without margin loan by accountId&lt;br&gt;&lt;br&gt;**Scope**: &#x60;transfers.read&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementBankingApi,
} from 'bezant-client';
import type { PostGwApiV1ExternalCashTransfersQueryOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementBankingApi();

  const body = {
    // PostGwApiV1ExternalCashTransfersQueryRequest | Create an external cash transfer query request body
    postGwApiV1ExternalCashTransfersQueryRequest: {"instruction":{"accountId":"U68903","clientInstructionId":7009007,"year":"2003"},"instructionType":"QUERY_IRA_CONTRIBUTIONS"},
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies PostGwApiV1ExternalCashTransfersQueryOperationRequest;

  try {
    const data = await api.postGwApiV1ExternalCashTransfersQuery(body);
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
| **postGwApiV1ExternalCashTransfersQueryRequest** | [PostGwApiV1ExternalCashTransfersQueryRequest](PostGwApiV1ExternalCashTransfersQueryRequest.md) | Create an external cash transfer query request body | |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**PostGwApiV1ExternalCashTransfersQuery201Response**](PostGwApiV1ExternalCashTransfersQuery201Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Instruction successfully created and processed synchronously |  -  |
| **400** | Returns a Problem detail instance representing a bad request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **422** | Returns a Problem detail instance representing a business error. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1InstructionsCancel

> SynchronousInstructionResponse postGwApiV1InstructionsCancel(postGwApiV1InstructionsCancelRequest, clientId)

Cancel Request

Cancel request by instructionId.&lt;br&gt;&lt;br&gt;**Scope**: &#x60;instructions.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementBankingApi,
} from 'bezant-client';
import type { PostGwApiV1InstructionsCancelOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementBankingApi();

  const body = {
    // PostGwApiV1InstructionsCancelRequest
    postGwApiV1InstructionsCancelRequest: {"instruction":{"clientInstructionId":12001810,"instructionId":43085477,"reason":"Testing"},"instructionType":"CANCEL_INSTRUCTION"},
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies PostGwApiV1InstructionsCancelOperationRequest;

  try {
    const data = await api.postGwApiV1InstructionsCancel(body);
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
| **postGwApiV1InstructionsCancelRequest** | [PostGwApiV1InstructionsCancelRequest](PostGwApiV1InstructionsCancelRequest.md) |  | |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**SynchronousInstructionResponse**](SynchronousInstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Instruction successfully created and processed synchronously |  -  |
| **400** | Returns a Problem detail instance representing a bad request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **422** | Returns a Problem detail instance representing a business error. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1InstructionsCancelBulk

> AsynchronousInstructionSetResponse postGwApiV1InstructionsCancelBulk(postGwApiV1InstructionsCancelBulkRequest, clientId)

Creates Multiple Cancel Instructions

&lt;br&gt;**Scope**: &#x60;instructions.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementBankingApi,
} from 'bezant-client';
import type { PostGwApiV1InstructionsCancelBulkOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementBankingApi();

  const body = {
    // PostGwApiV1InstructionsCancelBulkRequest | Create Bulk Cancel Instructions request body
    postGwApiV1InstructionsCancelBulkRequest: {"instructionType":"CANCEL_INSTRUCTION","instructions":[{"clientInstructionId":12001861,"instructionId":43085493,"reason":"Testing"},{"clientInstructionId":12001861,"instructionId":43085493,"reason":"Testing"},{"clientInstructionId":12001861,"instructionId":43085493,"reason":"Testing"}]},
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies PostGwApiV1InstructionsCancelBulkOperationRequest;

  try {
    const data = await api.postGwApiV1InstructionsCancelBulk(body);
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
| **postGwApiV1InstructionsCancelBulkRequest** | [PostGwApiV1InstructionsCancelBulkRequest](PostGwApiV1InstructionsCancelBulkRequest.md) | Create Bulk Cancel Instructions request body | |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**AsynchronousInstructionSetResponse**](AsynchronousInstructionSetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **202** | Accepts all instructions in Bulk request to create them asynchronously |  -  |
| **400** | Returns a Problem detail instance representing a bad request. Returned even if only one instruction in the bulk upload has syntactical errors. |  -  |
| **500** | Unable to process request due to an Internal Error. Please try again later. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1InstructionsQuery

> AsynchronousInstructionResponse postGwApiV1InstructionsQuery(postGwApiV1InstructionsQueryRequest, clientId)

Get Transaction History

Query list of recent transactions (up to 30 days) based on accountId.&lt;br&gt;&lt;br&gt;**Scope**: &#x60;instructions.read&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementBankingApi,
} from 'bezant-client';
import type { PostGwApiV1InstructionsQueryOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementBankingApi();

  const body = {
    // PostGwApiV1InstructionsQueryRequest | Create recent instructions request body
    postGwApiV1InstructionsQueryRequest: {"instruction":{"accountId":"U139838","clientInstructionId":7009001,"transactionHistory":{"daysToGoBack":3,"transactionType":"ACH_INSTRUCTION"}},"instructionType":"QUERY_RECENT_INSTRUCTIONS"},
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies PostGwApiV1InstructionsQueryOperationRequest;

  try {
    const data = await api.postGwApiV1InstructionsQuery(body);
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
| **postGwApiV1InstructionsQueryRequest** | [PostGwApiV1InstructionsQueryRequest](PostGwApiV1InstructionsQueryRequest.md) | Create recent instructions request body | |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**AsynchronousInstructionResponse**](AsynchronousInstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **202** | Accepts request to create a new instruction asynchronously |  -  |
| **400** | Returns a Problem detail instance representing a bad request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **422** | Returns a Problem detail instance representing a business error. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1InternalAssetTransfers

> AsynchronousInstructionResponse postGwApiV1InternalAssetTransfers(postGwApiV1InternalAssetTransfersRequest, clientId)

Transfer Positions Internally

Transfer positions internally between two accounts with Interactive Brokers&lt;br&gt;&lt;br&gt;**Scope**: &#x60;transfers.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementBankingApi,
} from 'bezant-client';
import type { PostGwApiV1InternalAssetTransfersOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementBankingApi();

  const body = {
    // PostGwApiV1InternalAssetTransfersRequest
    postGwApiV1InternalAssetTransfersRequest: {"instruction":{"clientInstructionId":7013044,"sourceAccountId":"U399192","targetAccountId":"U87440","tradingInstrument":{"currency":"USD","tradingInstrumentDescription":{"assetType":"STK","securityId":"459200101","securityIdType":"ISIN"}},"transferQuantity":6},"instructionType":"INTERNAL_POSITION_TRANSFER"},
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies PostGwApiV1InternalAssetTransfersOperationRequest;

  try {
    const data = await api.postGwApiV1InternalAssetTransfers(body);
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
| **postGwApiV1InternalAssetTransfersRequest** | [PostGwApiV1InternalAssetTransfersRequest](PostGwApiV1InternalAssetTransfersRequest.md) |  | |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**AsynchronousInstructionResponse**](AsynchronousInstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **202** | Accepts request to create a new instruction asynchronously |  -  |
| **400** | Returns a Problem detail instance representing a bad request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request. |  -  |
| **422** | Returns a Problem detail instance representing a business error. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1InternalAssetTransfersBulk

> AsynchronousInstructionSetResponse postGwApiV1InternalAssetTransfersBulk(postGwApiV1InternalAssetTransfersBulkRequest, clientId)

Creates Multiple Internal Asset Transfers Between The Provided Account Id Pairs

&lt;br&gt;**Scope**: &#x60;transfers.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementBankingApi,
} from 'bezant-client';
import type { PostGwApiV1InternalAssetTransfersBulkOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementBankingApi();

  const body = {
    // PostGwApiV1InternalAssetTransfersBulkRequest | Create Bulk Internal Asset Transfers request body
    postGwApiV1InternalAssetTransfersBulkRequest: {"instructionType":"INTERNAL_POSITION_TRANSFER","instructions":[{"clientInstructionId":7013005,"sourceAccountId":"U399192","targetAccountId":"U87440","tradingInstrument":{"conid":21323,"currency":"USD"},"transferQuantity":6},{"clientInstructionId":7013006,"sourceAccountId":"U399192","targetAccountId":"U87440","tradingInstrument":{"currency":"USD","tradingInstrumentDescription":{"assetType":"STK","securityId":"459200101","securityIdType":"ISIN"}},"transferQuantity":6},{"clientInstructionId":7013043,"settleDate":"2025-02-25","sourceAccountId":"U399192","targetAccountId":"U87440","tradeDate":"2025-02-17","tradingInstrument":{"conid":21323,"currency":"USD"},"transferPrice":100.3456789,"transferQuantity":6}]},
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies PostGwApiV1InternalAssetTransfersBulkOperationRequest;

  try {
    const data = await api.postGwApiV1InternalAssetTransfersBulk(body);
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
| **postGwApiV1InternalAssetTransfersBulkRequest** | [PostGwApiV1InternalAssetTransfersBulkRequest](PostGwApiV1InternalAssetTransfersBulkRequest.md) | Create Bulk Internal Asset Transfers request body | |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**AsynchronousInstructionSetResponse**](AsynchronousInstructionSetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **202** | Accepts all instructions in Bulk request to create them asynchronously |  -  |
| **400** | Returns a Problem detail instance representing a bad request. Returned even if only one instruction in the bulk upload has syntactical errors. |  -  |
| **500** | Unable to process request due to an Internal Error. Please try again later. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1InternalCashTransfers

> SynchronousInstructionResponse postGwApiV1InternalCashTransfers(postGwApiV1InternalCashTransfersRequest, clientId)

Transfer Cash Internally

Transfer cash internally between two accounts with Interactive Brokers.&lt;br&gt;&lt;br&gt;**Scope**: &#x60;transfers.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementBankingApi,
} from 'bezant-client';
import type { PostGwApiV1InternalCashTransfersOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementBankingApi();

  const body = {
    // PostGwApiV1InternalCashTransfersRequest
    postGwApiV1InternalCashTransfersRequest: {"instruction":{"amount":123.45,"clientInstructionId":1012983,"currency":"GBP","dateTimeToOccur":"2018-03-20T09:12:13Z","sourceAccountId":"U46377","targetAccountId":"U15667"},"instructionType":"INTERNAL_CASH_TRANSFER"},
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies PostGwApiV1InternalCashTransfersOperationRequest;

  try {
    const data = await api.postGwApiV1InternalCashTransfers(body);
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
| **postGwApiV1InternalCashTransfersRequest** | [PostGwApiV1InternalCashTransfersRequest](PostGwApiV1InternalCashTransfersRequest.md) |  | |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**SynchronousInstructionResponse**](SynchronousInstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Creates a new instruction synchronously, e.g., for Internal Cash Transfer w/o dateTimeToOccur |  -  |
| **400** | Returns a Problem detail instance representing a bad request. |  -  |
| **403** | Returns a Problem detail instance representing a forbidden request |  -  |
| **422** | Returns a Problem detail instance representing a business error. |  -  |
| **500** | Returns a Problem detail instance representing an internal server error. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## postGwApiV1InternalCashTransfersBulk

> AsynchronousInstructionSetResponse postGwApiV1InternalCashTransfersBulk(postGwApiV1InternalCashTransfersBulkRequest, clientId)

Creates Multiple Internal Cash Transfers Between The Provided Account Id Pairs

&lt;br&gt;**Scope**: &#x60;transfers.write&#x60;&lt;br&gt;**Security Policy**: &#x60;Signed JWT&#x60;

### Example

```ts
import {
  Configuration,
  AccountManagementBankingApi,
} from 'bezant-client';
import type { PostGwApiV1InternalCashTransfersBulkOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new AccountManagementBankingApi();

  const body = {
    // PostGwApiV1InternalCashTransfersBulkRequest | Create Bulk Internal Cash Transfers request body
    postGwApiV1InternalCashTransfersBulkRequest: {"instructionType":"INTERNAL_CASH_TRANSFER","instructions":[{"amount":123.45,"clientInstructionId":1,"currency":"GBP","dateTimeToOccur":"2018-03-20T09:12:13Z","sourceAccountId":"U345","targetAccountId":"U87440"},{"amount":123.45,"clientInstructionId":2,"currency":"GBP","dateTimeToOccur":"2018-03-20T09:12:13Z","sourceAccountId":"U345","targetAccountId":"U87440"},{"amount":123.45,"clientInstructionId":3,"currency":"GBP","dateTimeToOccur":"2018-03-20T09:12:13Z","sourceAccountId":"U399192","targetAccountId":"U87440"},{"amount":123.45,"clientInstructionId":4,"clientNote":"INT-CASH-TRANSFER:78965.38.15632","currency":"GBP","dateTimeToOccur":"2018-03-20T09:12:13Z","sourceAccountId":"U399192","targetAccountId":"U87440"}]},
    // string | The client\'s clientId (optional)
    clientId: abc123,
  } satisfies PostGwApiV1InternalCashTransfersBulkOperationRequest;

  try {
    const data = await api.postGwApiV1InternalCashTransfersBulk(body);
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
| **postGwApiV1InternalCashTransfersBulkRequest** | [PostGwApiV1InternalCashTransfersBulkRequest](PostGwApiV1InternalCashTransfersBulkRequest.md) | Create Bulk Internal Cash Transfers request body | |
| **clientId** | `string` | The client\&#39;s clientId | [Optional] [Defaults to `undefined`] |

### Return type

[**AsynchronousInstructionSetResponse**](AsynchronousInstructionSetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **202** | Accepts all instructions in Bulk request to create them asynchronously |  -  |
| **400** | Returns a Problem detail instance representing a bad request. Returned even if only one instruction in the bulk upload has syntactical errors. |  -  |
| **500** | Unable to process request due to an Internal Error. Please try again later. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

