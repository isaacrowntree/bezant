//! Ergonomic helpers layered on top of the generated client.
//!
//! These are the sugar that most rebalance bots end up reimplementing: page
//! walking, conid caching, etc. Nothing here changes the wire protocol — it's
//! all composed from the typed methods [`bezant_api::IbRestApiClient`] already
//! exposes.

use std::collections::HashMap;
use std::sync::Mutex;

use tracing::debug;

use crate::client::Client;
use crate::error::{Error, Result};

/// Positions returned for an account. Alias over the generated type so callers
/// can use `bezant::Position` without digging into `bezant_api`.
pub type Position = bezant_api::IndividualPosition;

/// Contract-search result, alias over the generated type.
pub type ContractSummary = bezant_api::SecdefSearchResponseSecdefSearchResponse;

/// Page size the Gateway returns for paginated position calls.
///
/// Exposed so CLIs / sidecars that replicate the page-walking loop don't
/// have to hard-code `30`. Tracks the size documented in CPAPI —
/// technically an IBKR-side constant rather than a bezant one, so if the
/// Gateway ever changes this we'll update the constant and bump the minor.
pub const POSITIONS_PAGE_SIZE: usize = 30;

/// Safety limit on the number of pages [`Client::all_positions`] will walk.
///
/// Three thousand positions is dramatically more than any realistic
/// rebalance-bot scope; this exists purely to stop a misbehaving Gateway
/// from spinning us forever.
pub const MAX_POSITION_PAGES: u32 = 100;

impl Client {
    /// Fetch every position across every page for `account_id`.
    ///
    /// CPAPI returns up to [`POSITIONS_PAGE_SIZE`] entries per page; this
    /// helper walks pages starting from 0 and stops once a short page (or
    /// an empty one) is returned. [`MAX_POSITION_PAGES`] caps runaway
    /// loops.
    ///
    /// # Errors
    /// Any transport / decode failure surfaces as [`Error`]. An
    /// [`Error::NotAuthenticated`] is returned if the Gateway reports the
    /// session is not live.
    #[tracing::instrument(skip(self), fields(account = %account_id), level = "debug")]
    pub async fn all_positions(&self, account_id: &str) -> Result<Vec<Position>> {
        let mut all = Vec::new();
        for page in 0..MAX_POSITION_PAGES {
            let req = bezant_api::GetPaginatedPositionsRequest {
                path: bezant_api::GetPaginatedPositionsRequestPath {
                    account_id: account_id.to_owned(),
                    // The generator models `page_id` as i32, so cast once
                    // at the request boundary rather than polluting the
                    // whole helper with a signed loop variable.
                    page_id: i32::try_from(page).unwrap_or(i32::MAX),
                },
                query: bezant_api::GetPaginatedPositionsRequestQuery::default(),
            };
            let resp = self.api().get_paginated_positions(req).await?;
            let batch = match resp {
                bezant_api::GetPaginatedPositionsResponse::Ok(items) => items,
                bezant_api::GetPaginatedPositionsResponse::Unauthorized => {
                    return Err(Error::NotAuthenticated)
                }
                bezant_api::GetPaginatedPositionsResponse::BadRequest => {
                    return Err(Error::other("bad request"))
                }
                bezant_api::GetPaginatedPositionsResponse::InternalServerError => {
                    return Err(Error::other("upstream 500"))
                }
                bezant_api::GetPaginatedPositionsResponse::ServiceUnavailable => {
                    return Err(Error::other("gateway service unavailable"))
                }
                bezant_api::GetPaginatedPositionsResponse::Unknown => {
                    return Err(Error::other("unknown upstream response"))
                }
            };
            let n = batch.len();
            debug!(page, fetched = n, "position page fetched");
            all.extend(batch);
            if n < POSITIONS_PAGE_SIZE {
                break;
            }
        }
        Ok(all)
    }
}

/// Symbol → conid cache.
///
/// Resolving a ticker to IBKR's numeric `conid` is a search call, and it's
/// stable across days — most bots do it once per ticker per session. Wrap
/// your [`Client`] with a [`SymbolCache`] to memoise lookups.
///
/// The cache is deliberately simple: a `Mutex<HashMap>`. It's built for the
/// low-volume rebalance-bot case (dozens of tickers, infrequent refreshes)
/// rather than high-volume quote streaming.
///
/// # Example
///
/// ```no_run
/// # async fn run() -> bezant::Result<()> {
/// let client = bezant::Client::new("https://localhost:5000/v1/api")?;
/// let cache = bezant::SymbolCache::new(client);
/// let aapl = cache.conid_for("AAPL").await?;
/// let msft = cache.conid_for("MSFT").await?;
/// // further calls for AAPL/MSFT hit the in-memory cache.
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct SymbolCache {
    client: Client,
    // `Option<i64>` so negative lookups (no such symbol) are remembered as
    // well — repeated typos hit the network exactly once.
    cache: Mutex<HashMap<String, Option<i64>>>,
}

/// Poisoned-mutex fallback: the protected state is a `HashMap<String, _>`
/// with no invariants beyond what the type system already gives us, so if
/// a panic ever poisoned the mutex we'd rather keep going than abort the
/// whole process — acquire the lock by taking the inner guard regardless.
fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

impl SymbolCache {
    /// Wrap a [`Client`] with an empty cache.
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self {
            client,
            cache: Mutex::new(HashMap::new()),
        }
    }

    /// Return the cached conid for `symbol`, looking it up on first miss.
    ///
    /// Only `STK`-type matches are cached. If you need options / bonds /
    /// futures, call [`Client::api`] directly. Both hits and misses are
    /// memoised — if a symbol turns out to be invalid, subsequent calls
    /// return the same [`Error::Other`] without touching the network.
    ///
    /// # Errors
    /// [`Error::Other`] if the symbol doesn't resolve to any contract,
    /// plus any transport / decode errors.
    #[tracing::instrument(skip(self), fields(symbol = %symbol), level = "debug")]
    pub async fn conid_for(&self, symbol: &str) -> Result<i64> {
        if let Some(entry) = lock(&self.cache).get(symbol).copied() {
            return match entry {
                Some(conid) => Ok(conid),
                None => Err(Error::other(format!(
                    "no contracts for symbol '{symbol}' (cached)"
                ))),
            };
        }

        let req = bezant_api::GetContractSymbolsFromBodyRequest {
            body: bezant_api::SearchRequestBody {
                symbol: symbol.to_owned(),
                sec_type: Some(bezant_api::GetContractSymbolsRequestQuerySecType::Stk),
                name: Some(false),
                more: Some(false),
                ..bezant_api::SearchRequestBody::default()
            },
        };
        let resp = self
            .client
            .api()
            .get_contract_symbols_from_body(req)
            .await?;
        let items = match resp {
            bezant_api::GetContractSymbolsResponse::Ok(items) => items,
            bezant_api::GetContractSymbolsResponse::BadRequest => {
                // BadRequest means the symbol itself is malformed — cache
                // the negative so we don't hit the CDN over and over for
                // a caller that's retrying.
                lock(&self.cache).insert(symbol.to_owned(), None);
                return Err(Error::other(format!("bad symbol: {symbol}")));
            }
            bezant_api::GetContractSymbolsResponse::Unauthorized => {
                return Err(Error::NotAuthenticated)
            }
            bezant_api::GetContractSymbolsResponse::InternalServerError => {
                return Err(Error::other("upstream 500"))
            }
            bezant_api::GetContractSymbolsResponse::ServiceUnavailable => {
                return Err(Error::other("gateway service unavailable"))
            }
            bezant_api::GetContractSymbolsResponse::Unknown => {
                return Err(Error::other("unknown upstream response"))
            }
        };
        let Some(first) = items.first() else {
            lock(&self.cache).insert(symbol.to_owned(), None);
            return Err(Error::other(format!(
                "no contracts for symbol '{symbol}'"
            )));
        };
        let conid_str = first
            .conid
            .as_deref()
            .ok_or_else(|| Error::other(format!("contract for '{symbol}' has no conid")))?;
        let conid: i64 = conid_str
            .parse()
            .map_err(|e| Error::other(format!("invalid conid '{conid_str}': {e}")))?;

        lock(&self.cache).insert(symbol.to_owned(), Some(conid));
        Ok(conid)
    }

    /// Drop a single entry — useful after IBKR restructures a listing.
    pub fn forget(&self, symbol: &str) {
        lock(&self.cache).remove(symbol);
    }

    /// Clear the whole cache.
    pub fn clear(&self) {
        lock(&self.cache).clear();
    }

    /// Borrow the inner [`Client`] — useful when callers want both the cache
    /// and direct API access from the same instance.
    #[must_use]
    pub fn client(&self) -> &Client {
        &self.client
    }
}
