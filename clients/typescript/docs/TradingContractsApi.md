# TradingContractsApi

All URIs are relative to *https://api.ibkr.com*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**getAlgosByInstrument**](TradingContractsApi.md#getalgosbyinstrument) | **GET** /iserver/contract/{conid}/algos | Search Algos For An Instrument |
| [**getBondFilters**](TradingContractsApi.md#getbondfilters) | **GET** /iserver/secdef/bond-filters | Search Bond Filter Information |
| [**getConidsByExchange**](TradingContractsApi.md#getconidsbyexchange) | **GET** /trsrv/all-conids | List All Stock Conids By Exchange |
| [**getContractInfo**](TradingContractsApi.md#getcontractinfo) | **GET** /iserver/secdef/info | Instrument Attributes Detail |
| [**getContractRules**](TradingContractsApi.md#getcontractrulesoperation) | **POST** /iserver/contract/rules | Search Contract Rules |
| [**getContractStrikes**](TradingContractsApi.md#getcontractstrikes) | **GET** /iserver/secdef/strikes | Search Strikes For An Underlier |
| [**getContractSymbols**](TradingContractsApi.md#getcontractsymbols) | **GET** /iserver/secdef/search | Search Instruments By Symbol |
| [**getContractSymbolsFromBody**](TradingContractsApi.md#getcontractsymbolsfrombodyoperation) | **POST** /iserver/secdef/search | Search Instruments By Symbol |
| [**getCurrencyPairs**](TradingContractsApi.md#getcurrencypairs) | **GET** /iserver/currency/pairs | Available Currency Pairs |
| [**getExchangeRates**](TradingContractsApi.md#getexchangerates) | **GET** /iserver/exchangerate | Currency Exchange Rate |
| [**getFutureBySymbol**](TradingContractsApi.md#getfuturebysymbol) | **GET** /trsrv/futures | Search Futures By Symbol |
| [**getInfoAndRules**](TradingContractsApi.md#getinfoandrules) | **GET** /iserver/contract/{conid}/info-and-rules | Instrument Info And Market Rules |
| [**getInstrumentDefinition**](TradingContractsApi.md#getinstrumentdefinition) | **GET** /trsrv/secdef | Instrument Definition Detail |
| [**getInstrumentInfo**](TradingContractsApi.md#getinstrumentinfo) | **GET** /iserver/contract/{conid}/info | General Instrument Information |
| [**getStockBySymbol**](TradingContractsApi.md#getstockbysymbol) | **GET** /trsrv/stocks | Search Stocks By Symbol |
| [**getTradingSchedule**](TradingContractsApi.md#gettradingschedule) | **GET** /contract/trading-schedule | Trading Schedule (NEW) |
| [**getTradingScheduleGetTrsrvSecdefSchedule**](TradingContractsApi.md#gettradingschedulegettrsrvsecdefschedule) | **GET** /trsrv/secdef/schedule | Trading Schedule By Symbol |



## getAlgosByInstrument

> AlgosResponse getAlgosByInstrument(conid, algos, addDescription, addParams)

Search Algos For An Instrument

Returns supported IB Algos for an instrument. A pre-flight request must be submitted before retrieving information.

### Example

```ts
import {
  Configuration,
  TradingContractsApi,
} from 'bezant-client';
import type { GetAlgosByInstrumentRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingContractsApi();

  const body = {
    // string
    conid: conid_example,
    // 'Adaptive' | 'Vwap' (optional)
    algos: algos_example,
    // 0 | 1 (optional)
    addDescription: 56,
    // 0 | 1 (optional)
    addParams: 56,
  } satisfies GetAlgosByInstrumentRequest;

  try {
    const data = await api.getAlgosByInstrument(body);
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
| **conid** | `string` |  | [Defaults to `undefined`] |
| **algos** | `Adaptive`, `Vwap` |  | [Optional] [Defaults to `undefined`] [Enum: Adaptive, Vwap] |
| **addDescription** | `0`, `1` |  | [Optional] [Defaults to `0`] [Enum: 0, 1] |
| **addParams** | `0`, `1` |  | [Optional] [Defaults to `0`] [Enum: 0, 1] |

### Return type

[**AlgosResponse**](AlgosResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns a list of available algos and a description of their behavior. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getBondFilters

> BondFiltersResponse getBondFilters(symbol, issuerId)

Search Bond Filter Information

Request a list of filters relating to a given Bond issuerID. The issuerId is retrieved from /iserver/secdef/search and can be used in /iserver/secdef/info?issuerId&#x3D;{issuerId} for retrieving conIds.

### Example

```ts
import {
  Configuration,
  TradingContractsApi,
} from 'bezant-client';
import type { GetBondFiltersRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingContractsApi();

  const body = {
    // string
    symbol: BOND,
    // string
    issuerId: e1400715,
  } satisfies GetBondFiltersRequest;

  try {
    const data = await api.getBondFilters(body);
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
| **symbol** | `string` |  | [Defaults to `undefined`] |
| **issuerId** | `string` |  | [Defaults to `undefined`] |

### Return type

[**BondFiltersResponse**](BondFiltersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successful requests return the currency exchange rate of the target currency value divided by the source currency. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getConidsByExchange

> Array&lt;GetConidsByExchange200ResponseInner&gt; getConidsByExchange(exchange, assetClass)

List All Stock Conids By Exchange

Send out a request to retrieve all contracts made available on a requested exchange. This returns all contracts that are tradable on the exchange, even those that are not using the exchange as their primary listing.

### Example

```ts
import {
  Configuration,
  TradingContractsApi,
} from 'bezant-client';
import type { GetConidsByExchangeRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingContractsApi();

  const body = {
    // string
    exchange: AMEX,
    // any (optional)
    assetClass: ...,
  } satisfies GetConidsByExchangeRequest;

  try {
    const data = await api.getConidsByExchange(body);
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
| **exchange** | `string` |  | [Defaults to `undefined`] |
| **assetClass** | `any` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**Array&lt;GetConidsByExchange200ResponseInner&gt;**](GetConidsByExchange200ResponseInner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successful response containing a contract\&#39;s security definition. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getContractInfo

> SecDefInfoResponse getContractInfo(conid, sectype, month, exchange, strike, right, issuerId, filters)

Instrument Attributes Detail

Returns the attributes of the instrument.

### Example

```ts
import {
  Configuration,
  TradingContractsApi,
} from 'bezant-client';
import type { GetContractInfoRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingContractsApi();

  const body = {
    // string (optional)
    conid: 265598,
    // any (optional)
    sectype: ...,
    // any (optional)
    month: ...,
    // any (optional)
    exchange: ...,
    // any (optional)
    strike: ...,
    // 'C' | 'P' (optional)
    right: right_example,
    // string (optional)
    issuerId: e1234567,
    // any (optional)
    filters: ...,
  } satisfies GetContractInfoRequest;

  try {
    const data = await api.getContractInfo(body);
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
| **conid** | `string` |  | [Optional] [Defaults to `undefined`] |
| **sectype** | `any` |  | [Optional] [Defaults to `undefined`] |
| **month** | `any` |  | [Optional] [Defaults to `undefined`] |
| **exchange** | `any` |  | [Optional] [Defaults to `undefined`] |
| **strike** | `any` |  | [Optional] [Defaults to `undefined`] |
| **right** | `C`, `P` |  | [Optional] [Defaults to `undefined`] [Enum: C, P] |
| **issuerId** | `string` |  | [Optional] [Defaults to `undefined`] |
| **filters** | `any` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**SecDefInfoResponse**](SecDefInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successful response containing a contract\&#39;s security definition. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getContractRules

> ContractRules getContractRules(getContractRulesRequest)

Search Contract Rules

Returns trading related rules for a specific contract and side.

### Example

```ts
import {
  Configuration,
  TradingContractsApi,
} from 'bezant-client';
import type { GetContractRulesOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingContractsApi();

  const body = {
    // GetContractRulesRequest
    getContractRulesRequest: ...,
  } satisfies GetContractRulesOperationRequest;

  try {
    const data = await api.getContractRules(body);
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
| **getContractRulesRequest** | [GetContractRulesRequest](GetContractRulesRequest.md) |  | |

### Return type

[**ContractRules**](ContractRules.md)

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


## getContractStrikes

> GetContractStrikes200Response getContractStrikes(conid, sectype, month, exchange)

Search Strikes For An Underlier

Returns lists of valid strikes for options contracts on a given underlier, for all currently trading expirations. The /iserver/secdef/search endpoint must be called prior for the underlying. Otherwise empty arrays will return for \&quot;puts\&quot; and \&quot;calls\&quot;. 

### Example

```ts
import {
  Configuration,
  TradingContractsApi,
} from 'bezant-client';
import type { GetContractStrikesRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingContractsApi();

  const body = {
    // string
    conid: 265598,
    // 'OPT' | 'FOP' | 'WAR'
    sectype: sectype_example,
    // string
    month: JAN24,
    // string (optional)
    exchange: CME,
  } satisfies GetContractStrikesRequest;

  try {
    const data = await api.getContractStrikes(body);
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
| **conid** | `string` |  | [Defaults to `undefined`] |
| **sectype** | `OPT`, `FOP`, `WAR` |  | [Defaults to `undefined`] [Enum: OPT, FOP, WAR] |
| **month** | `string` |  | [Defaults to `undefined`] |
| **exchange** | `string` |  | [Optional] [Defaults to `&#39;SMART&#39;`] |

### Return type

[**GetContractStrikes200Response**](GetContractStrikes200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successful response containing a contract\&#39;s security definition. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getContractSymbols

> Array&lt;SecdefSearchResponseInner&gt; getContractSymbols(symbol, secType, name, more, fund, fundFamilyConidEx, pattern, referrer)

Search Instruments By Symbol

Returns a list of contracts based on the search symbol provided as a query param.

### Example

```ts
import {
  Configuration,
  TradingContractsApi,
} from 'bezant-client';
import type { GetContractSymbolsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingContractsApi();

  const body = {
    // string (optional)
    symbol: AAPL,
    // 'STK' | 'IND' | 'BOND' | Available underlying security types:   * `STK` - Represents an underlying as a Stock security type.   * `IND` - Represents an underlying as an Index security type.   * `BOND` - Represents an underlying as a Bond security type.  (optional)
    secType: secType_example,
    // boolean (optional)
    name: true,
    // boolean (optional)
    more: true,
    // boolean (optional)
    fund: true,
    // string (optional)
    fundFamilyConidEx: fundFamilyConidEx_example,
    // boolean (optional)
    pattern: true,
    // string (optional)
    referrer: referrer_example,
  } satisfies GetContractSymbolsRequest;

  try {
    const data = await api.getContractSymbols(body);
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
| **symbol** | `string` |  | [Optional] [Defaults to `undefined`] |
| **secType** | `STK`, `IND`, `BOND` | Available underlying security types:   * &#x60;STK&#x60; - Represents an underlying as a Stock security type.   * &#x60;IND&#x60; - Represents an underlying as an Index security type.   * &#x60;BOND&#x60; - Represents an underlying as a Bond security type.  | [Optional] [Defaults to `&#39;STK&#39;`] [Enum: STK, IND, BOND] |
| **name** | `boolean` |  | [Optional] [Defaults to `undefined`] |
| **more** | `boolean` |  | [Optional] [Defaults to `undefined`] |
| **fund** | `boolean` |  | [Optional] [Defaults to `undefined`] |
| **fundFamilyConidEx** | `string` |  | [Optional] [Defaults to `undefined`] |
| **pattern** | `boolean` |  | [Optional] [Defaults to `undefined`] |
| **referrer** | `string` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**Array&lt;SecdefSearchResponseInner&gt;**](SecdefSearchResponseInner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | An array of objects detailing contract information. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getContractSymbolsFromBody

> Array&lt;SecdefSearchResponseInner&gt; getContractSymbolsFromBody(getContractSymbolsFromBodyRequest)

Search Instruments By Symbol

Returns a list of contracts based on the search symbol provided as a query param.

### Example

```ts
import {
  Configuration,
  TradingContractsApi,
} from 'bezant-client';
import type { GetContractSymbolsFromBodyOperationRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingContractsApi();

  const body = {
    // GetContractSymbolsFromBodyRequest
    getContractSymbolsFromBodyRequest: ...,
  } satisfies GetContractSymbolsFromBodyOperationRequest;

  try {
    const data = await api.getContractSymbolsFromBody(body);
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
| **getContractSymbolsFromBodyRequest** | [GetContractSymbolsFromBodyRequest](GetContractSymbolsFromBodyRequest.md) |  | |

### Return type

[**Array&lt;SecdefSearchResponseInner&gt;**](SecdefSearchResponseInner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | An array of objects detailing contract information. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getCurrencyPairs

> { [key: string]: Array&lt;CurrencyPairsValueInner&gt;; } getCurrencyPairs(currency)

Available Currency Pairs

Obtains available currency pairs corresponding to the given target currency.

### Example

```ts
import {
  Configuration,
  TradingContractsApi,
} from 'bezant-client';
import type { GetCurrencyPairsRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingContractsApi();

  const body = {
    // string
    currency: USD,
  } satisfies GetCurrencyPairsRequest;

  try {
    const data = await api.getCurrencyPairs(body);
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
| **currency** | `string` |  | [Defaults to `undefined`] |

### Return type

**{ [key: string]: Array<CurrencyPairsValueInner>; }**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns a list of valid forex pairs for the given currency. The currency can apply as both the target or base currency. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getExchangeRates

> GetExchangeRates200Response getExchangeRates(target, source)

Currency Exchange Rate

Obtains the exchange rates of the currency pair.

### Example

```ts
import {
  Configuration,
  TradingContractsApi,
} from 'bezant-client';
import type { GetExchangeRatesRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingContractsApi();

  const body = {
    // string
    target: EUR,
    // string
    source: USD,
  } satisfies GetExchangeRatesRequest;

  try {
    const data = await api.getExchangeRates(body);
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
| **target** | `string` |  | [Defaults to `undefined`] |
| **source** | `string` |  | [Defaults to `undefined`] |

### Return type

[**GetExchangeRates200Response**](GetExchangeRates200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successful requests return the currency exchange rate of the target currency value divided by the source currency. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getFutureBySymbol

> Features getFutureBySymbol(symbols, exchange)

Search Futures By Symbol

Returns a list of non-expired future contracts for given symbol(s)

### Example

```ts
import {
  Configuration,
  TradingContractsApi,
} from 'bezant-client';
import type { GetFutureBySymbolRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingContractsApi();

  const body = {
    // string
    symbols: ES,MES,
    // string (optional)
    exchange: CME,
  } satisfies GetFutureBySymbolRequest;

  try {
    const data = await api.getFutureBySymbol(body);
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
| **symbols** | `string` |  | [Defaults to `undefined`] |
| **exchange** | `string` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**Features**](Features.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successful response containing a contract\&#39;s security definition. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getInfoAndRules

> GetInfoAndRules200Response getInfoAndRules(conid)

Instrument Info And Market Rules

Requests full contract details and trading rules for the given conid. A follow-up request will provide additional trading rules. 

### Example

```ts
import {
  Configuration,
  TradingContractsApi,
} from 'bezant-client';
import type { GetInfoAndRulesRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingContractsApi();

  const body = {
    // string
    conid: 265598,
  } satisfies GetInfoAndRulesRequest;

  try {
    const data = await api.getInfoAndRules(body);
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
| **conid** | `string` |  | [Defaults to `undefined`] |

### Return type

[**GetInfoAndRules200Response**](GetInfoAndRules200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns all contract information and trading rules for the contract. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getInstrumentDefinition

> TrsrvSecDefResponse getInstrumentDefinition(conids)

Instrument Definition Detail

Returns a list of security definitions for the given conids.

### Example

```ts
import {
  Configuration,
  TradingContractsApi,
} from 'bezant-client';
import type { GetInstrumentDefinitionRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingContractsApi();

  const body = {
    // string
    conids: 265598,8314,
  } satisfies GetInstrumentDefinitionRequest;

  try {
    const data = await api.getInstrumentDefinition(body);
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
| **conids** | `string` |  | [Defaults to `undefined`] |

### Return type

[**TrsrvSecDefResponse**](TrsrvSecDefResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successful response containing a contract\&#39;s security definition. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getInstrumentInfo

> ContractInfo getInstrumentInfo(conid)

General Instrument Information

Requests full contract details for the given conid.

### Example

```ts
import {
  Configuration,
  TradingContractsApi,
} from 'bezant-client';
import type { GetInstrumentInfoRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingContractsApi();

  const body = {
    // string
    conid: 265598,
  } satisfies GetInstrumentInfoRequest;

  try {
    const data = await api.getInstrumentInfo(body);
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
| **conid** | `string` |  | [Defaults to `undefined`] |

### Return type

[**ContractInfo**](ContractInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns detailed information for the passed contract. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getStockBySymbol

> { [key: string]: Array&lt;StocksValueInner&gt;; } getStockBySymbol(symbols)

Search Stocks By Symbol

Returns an object contains all stock contracts for given symbol(s)

### Example

```ts
import {
  Configuration,
  TradingContractsApi,
} from 'bezant-client';
import type { GetStockBySymbolRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingContractsApi();

  const body = {
    // string
    symbols: AAPL,IBKR,
  } satisfies GetStockBySymbolRequest;

  try {
    const data = await api.getStockBySymbol(body);
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
| **symbols** | `string` |  | [Defaults to `undefined`] |

### Return type

**{ [key: string]: Array<StocksValueInner>; }**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Successful response containing a contract\&#39;s security definition. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getTradingSchedule

> ScheduleResponse getTradingSchedule(conid, exchange)

Trading Schedule (NEW)

Returns the trading schedule for the 6 total days surrounding the current trading day. Non-Trading days, such as holidays, will not be returned.

### Example

```ts
import {
  Configuration,
  TradingContractsApi,
} from 'bezant-client';
import type { GetTradingScheduleRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingContractsApi();

  const body = {
    // string
    conid: conid_example,
    // string (optional)
    exchange: exchange_example,
  } satisfies GetTradingScheduleRequest;

  try {
    const data = await api.getTradingSchedule(body);
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
| **conid** | `string` |  | [Defaults to `undefined`] |
| **exchange** | `string` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**ScheduleResponse**](ScheduleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns the liquid and extended trading hours of the coming and prior trading days. |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getTradingScheduleGetTrsrvSecdefSchedule

> Array&lt;TradingScheduleInner&gt; getTradingScheduleGetTrsrvSecdefSchedule(assetClass, symbol, exchange, exchangeFilter)

Trading Schedule By Symbol

Returns the trading schedule up to a month for the requested contract.

### Example

```ts
import {
  Configuration,
  TradingContractsApi,
} from 'bezant-client';
import type { GetTradingScheduleGetTrsrvSecdefScheduleRequest } from 'bezant-client';

async function example() {
  console.log("🚀 Testing bezant-client SDK...");
  const api = new TradingContractsApi();

  const body = {
    // 'STK' | 'OPT' | 'FUT' | 'CFD' | 'WAR' | 'SWP' | 'FND' | 'BND' | 'ICS'
    assetClass: assetClass_example,
    // string
    symbol: AAPL,
    // string (optional)
    exchange: NASDAQ,
    // string (optional)
    exchangeFilter: AMEX,NASDAQ,NYSE,
  } satisfies GetTradingScheduleGetTrsrvSecdefScheduleRequest;

  try {
    const data = await api.getTradingScheduleGetTrsrvSecdefSchedule(body);
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
| **assetClass** | `STK`, `OPT`, `FUT`, `CFD`, `WAR`, `SWP`, `FND`, `BND`, `ICS` |  | [Defaults to `undefined`] [Enum: STK, OPT, FUT, CFD, WAR, SWP, FND, BND, ICS] |
| **symbol** | `string` |  | [Defaults to `undefined`] |
| **exchange** | `string` |  | [Optional] [Defaults to `undefined`] |
| **exchangeFilter** | `string` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**Array&lt;TradingScheduleInner&gt;**](TradingScheduleInner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns the trading schedule up to a month for the requested contract. |  -  |
| **400** | Invalid input format or missing required fields |  -  |
| **401** | Invalid or expired access token |  -  |
| **500** | internal server error, returned when incoming request cannot be processed. It can sometimes include subset of bad requests.  For example, wrong accountId passed and it can only be detected later in handling request. Error contains reason of the problem.  |  -  |
| **503** | service is unavailable. For example if request takes more than 10s due to some internal service unavailability,  request aborted and this status returned  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

