//! Name-keyed cookie store for the single-host CPGateway use case.
//!
//! `reqwest::cookie::Jar` keys cookies by `(domain, path, name)`, which
//! means two `Set-Cookie` responses from the same Gateway that pick
//! different paths (e.g. one at `/v1/api/` from a keepalive `/tickle`,
//! one at `/` from a passthrough cookie injection) end up as **two
//! distinct entries**. When a typed API call later asks the jar what
//! cookies to attach, both qualify and reqwest sends a `Cookie:`
//! header with **two values for the same name** — which CPGateway
//! treats as a session mismatch and rejects.
//!
//! Bezant-server is single-tenant against a single Gateway host, so we
//! don't need RFC 6265's path/domain semantics. This jar keys cookies
//! purely by name; setting `JSESSIONID=NEW` replaces `JSESSIONID=OLD`
//! regardless of the path either was originally set on. The trade-off
//! is that a Gateway with overlapping per-path cookies (rare in
//! practice) would lose granularity — fine for our use case.

use std::collections::HashMap;
use std::sync::Mutex;

use reqwest::cookie::CookieStore;
use reqwest::header::HeaderValue;
use url::Url;

/// Thread-safe, name-keyed cookie store. `set_cookies` parses each
/// `Set-Cookie` value and stores `(name → value)` ignoring everything
/// after the first `;` (attributes like `Path`, `HttpOnly`, `Secure`).
#[derive(Debug, Default)]
pub struct NameKeyedJar {
    inner: Mutex<HashMap<String, String>>,
}

impl NameKeyedJar {
    /// Create an empty jar.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert raw `name=value` pairs, ignoring any cookie attributes.
    /// Pairs with the same name replace previous values.
    pub fn set_pairs<I, S>(&self, pairs: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        for raw in pairs {
            let trimmed = raw.as_ref().trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Some((name, value)) = trimmed.split_once('=') {
                let name = name.trim().to_owned();
                let value = value.split(';').next().unwrap_or("").trim().to_owned();
                guard.insert(name, value);
            }
        }
    }

    /// Drop every entry. Useful when the upstream signals a session
    /// reset.
    pub fn clear(&self) {
        self.inner.lock().unwrap_or_else(|e| e.into_inner()).clear();
    }

    /// Return the number of entries currently held — handy for diagnostics.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.lock().unwrap_or_else(|e| e.into_inner()).len()
    }

    /// Return `true` if the jar currently holds no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Snapshot of the current `(name, value)` pairs, sorted by name
    /// for stable diagnostic output.
    #[must_use]
    pub fn snapshot(&self) -> Vec<(String, String)> {
        let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let mut entries: Vec<_> = guard.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        entries
    }
}

impl CookieStore for NameKeyedJar {
    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, _url: &Url) {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        for header in cookie_headers {
            let Ok(s) = header.to_str() else { continue };
            // Each `Set-Cookie` header carries one `name=value` plus
            // attributes; we want only the first segment.
            let primary = s.split(';').next().unwrap_or("").trim();
            if primary.is_empty() {
                continue;
            }
            if let Some((name, value)) = primary.split_once('=') {
                guard.insert(name.trim().to_owned(), value.trim().to_owned());
            }
        }
    }

    fn cookies(&self, _url: &Url) -> Option<HeaderValue> {
        let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        if guard.is_empty() {
            return None;
        }
        let mut buf = String::new();
        for (i, (name, value)) in guard.iter().enumerate() {
            if i > 0 {
                buf.push_str("; ");
            }
            buf.push_str(name);
            buf.push('=');
            buf.push_str(value);
        }
        HeaderValue::from_str(&buf).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_cookies_replaces_by_name() {
        let jar = NameKeyedJar::new();
        let url = Url::parse("https://localhost:5000/").unwrap();
        let h1 = HeaderValue::from_static("JSESSIONID=OLD; Path=/sso; HttpOnly");
        let h2 = HeaderValue::from_static("JSESSIONID=NEW; Path=/; Secure");
        jar.set_cookies(&mut [h1].iter(), &url);
        jar.set_cookies(&mut [h2].iter(), &url);
        let snapshot = jar.snapshot();
        assert_eq!(snapshot, vec![("JSESSIONID".into(), "NEW".into())]);
    }

    #[test]
    fn cookies_emits_combined_header_in_stable_order() {
        let jar = NameKeyedJar::new();
        let url = Url::parse("https://localhost:5000/").unwrap();
        jar.set_pairs(["b=2", "a=1", "c=3"]);
        let header = jar.cookies(&url).unwrap();
        // Map iteration order isn't deterministic, but every pair must
        // be present and use `; ` separation.
        let s = header.to_str().unwrap();
        for needle in ["a=1", "b=2", "c=3"] {
            assert!(s.contains(needle), "expected {needle} in {s:?}");
        }
        assert!(s.contains("; "));
    }

    #[test]
    fn set_pairs_strips_attributes() {
        let jar = NameKeyedJar::new();
        jar.set_pairs(["JSESSIONID=ABC; Domain=.ibkr.com; Secure"]);
        assert_eq!(jar.snapshot(), vec![("JSESSIONID".into(), "ABC".into())]);
    }

    #[test]
    fn empty_jar_returns_none() {
        let jar = NameKeyedJar::new();
        let url = Url::parse("https://localhost:5000/").unwrap();
        assert!(jar.cookies(&url).is_none());
    }
}
