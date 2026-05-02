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
use std::fmt;
use std::sync::Mutex;

use reqwest::cookie::CookieStore;
use reqwest::header::HeaderValue;
use url::Url;

/// Thread-safe, name-keyed cookie store. `set_cookies` parses each
/// `Set-Cookie` value and stores `(name → value)` ignoring everything
/// after the first `;` (attributes like `Path`, `HttpOnly`, `Secure`).
#[derive(Default)]
pub struct NameKeyedJar {
    // Held only across sub-microsecond `HashMap` ops; never across
    // `.await`. `std::sync::Mutex` is the right choice here —
    // `tokio::sync::Mutex` would force `.await` and offer no benefit.
    inner: Mutex<HashMap<String, String>>,
}

impl fmt::Debug for NameKeyedJar {
    /// Print the cookie *names* and the entry count, never the values.
    /// Cookie values frequently carry secrets (JSESSIONID, OAuth
    /// tokens) and a derived `Debug` would leak them through every
    /// `tracing::debug!(?jar)` call site.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let mut names: Vec<&str> = guard.keys().map(String::as_str).collect();
        names.sort_unstable();
        f.debug_struct("NameKeyedJar")
            .field("len", &guard.len())
            .field("names", &names)
            .finish()
    }
}

/// True when the byte is a CR/LF/NUL or other control char that
/// `HeaderValue::from_str` would refuse, or a structural character
/// that would split the cookie pair when serialised. Used to reject
/// malformed Set-Cookie values before they poison the jar.
fn is_unsafe_byte(b: u8) -> bool {
    b < 0x20 || b == 0x7F
}

fn is_safe(s: &str) -> bool {
    !s.bytes().any(is_unsafe_byte)
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
                let name = name.trim();
                let value = value.split(';').next().unwrap_or("").trim();
                // Reject names/values containing control chars or
                // delimiter characters that would either be rejected
                // by `HeaderValue::from_str` later (silently dropping
                // the whole `Cookie:` header) or break the
                // `name1=v1; name2=v2` serialisation we emit from
                // `cookies()`.
                if name.is_empty() || !is_safe(name) || !is_safe(value) {
                    continue;
                }
                guard.insert(name.to_owned(), value.to_owned());
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
                let name = name.trim();
                let value = value.trim();
                // Same hardening as `set_pairs` — reject control chars
                // and empty names so a single malformed Set-Cookie
                // can't poison subsequent `Cookie:` serialisation.
                if name.is_empty() || !is_safe(name) || !is_safe(value) {
                    continue;
                }
                guard.insert(name.to_owned(), value.to_owned());
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

    fn url() -> Url {
        Url::parse("https://localhost:5000/").unwrap()
    }

    #[test]
    fn set_cookies_replaces_by_name() {
        let jar = NameKeyedJar::new();
        let h1 = HeaderValue::from_static("JSESSIONID=OLD; Path=/sso; HttpOnly");
        let h2 = HeaderValue::from_static("JSESSIONID=NEW; Path=/; Secure");
        jar.set_cookies(&mut [h1].iter(), &url());
        jar.set_cookies(&mut [h2].iter(), &url());
        assert_eq!(jar.snapshot(), vec![("JSESSIONID".into(), "NEW".into())]);
    }

    #[test]
    fn set_pairs_replaces_by_name() {
        let jar = NameKeyedJar::new();
        jar.set_pairs(["X=A"]);
        jar.set_pairs(["X=B"]);
        assert_eq!(jar.snapshot(), vec![("X".into(), "B".into())]);
    }

    #[test]
    fn set_pairs_strips_attributes() {
        let jar = NameKeyedJar::new();
        jar.set_pairs(["JSESSIONID=ABC; Domain=.ibkr.com; Secure"]);
        assert_eq!(jar.snapshot(), vec![("JSESSIONID".into(), "ABC".into())]);
    }

    #[test]
    fn cookies_combines_pairs_with_semicolon_separator() {
        let jar = NameKeyedJar::new();
        jar.set_pairs(["b=2", "a=1", "c=3"]);
        // `HashMap` iteration order isn't stable, so assert presence
        // + separator instead of full equality.
        let s = jar.cookies(&url()).unwrap().to_str().unwrap().to_owned();
        for needle in ["a=1", "b=2", "c=3"] {
            assert!(s.contains(needle), "expected {needle} in {s:?}");
        }
        assert_eq!(s.matches("; ").count(), 2);
    }

    #[test]
    fn cookies_single_entry_has_no_trailing_separator() {
        let jar = NameKeyedJar::new();
        jar.set_pairs(["only=one"]);
        let header = jar.cookies(&url()).unwrap();
        assert_eq!(header.to_str().unwrap(), "only=one");
    }

    #[test]
    fn empty_jar_returns_none() {
        let jar = NameKeyedJar::new();
        assert!(jar.cookies(&url()).is_none());
        assert!(jar.is_empty());
        assert_eq!(jar.len(), 0);
    }

    #[test]
    fn clear_empties_jar() {
        let jar = NameKeyedJar::new();
        jar.set_pairs(["a=1", "b=2"]);
        assert_eq!(jar.len(), 2);
        jar.clear();
        assert!(jar.is_empty());
        assert!(jar.cookies(&url()).is_none());
    }

    #[test]
    fn snapshot_returns_sorted_by_name() {
        let jar = NameKeyedJar::new();
        jar.set_pairs(["c=3", "a=1", "b=2"]);
        assert_eq!(
            jar.snapshot(),
            vec![
                ("a".into(), "1".into()),
                ("b".into(), "2".into()),
                ("c".into(), "3".into()),
            ]
        );
    }

    #[test]
    fn set_cookies_handles_iterator_of_multiple_values() {
        let jar = NameKeyedJar::new();
        let h1 = HeaderValue::from_static("a=1; Path=/");
        let h2 = HeaderValue::from_static("b=2; HttpOnly");
        let h3 = HeaderValue::from_static("c=3");
        let mut iter = [&h1, &h2, &h3].into_iter();
        jar.set_cookies(&mut iter, &url());
        assert_eq!(jar.len(), 3);
    }

    #[test]
    fn set_cookies_skips_non_ascii_header_values() {
        let jar = NameKeyedJar::new();
        // Non-UTF8 byte → `to_str()` fails and we skip silently.
        let bad = HeaderValue::from_bytes(b"name=\xff").unwrap();
        let good = HeaderValue::from_static("kept=yes");
        jar.set_cookies(&mut [&bad, &good].into_iter(), &url());
        assert_eq!(jar.snapshot(), vec![("kept".into(), "yes".into())]);
    }

    #[test]
    fn set_pairs_skips_malformed_inputs() {
        let jar = NameKeyedJar::new();
        jar.set_pairs([
            "",           // empty
            "   ",        // whitespace only
            "novalue",    // no `=`
            "=onlyvalue", // empty name
            "name=",      // empty value (allowed)
            "n=a=b",      // value with `=`, kept as-is up to the first `;`
        ]);
        let snap = jar.snapshot();
        // `name=` produces ("name", "") — empty value is legal cookie state.
        // `n=a=b` produces ("n", "a=b") — split_once stops at first `=`.
        assert!(snap.contains(&("name".into(), String::new())));
        assert!(snap.contains(&("n".into(), "a=b".into())));
        assert_eq!(snap.len(), 2);
    }

    #[test]
    fn set_pairs_rejects_crlf_injection_in_value() {
        let jar = NameKeyedJar::new();
        // `HeaderValue` validation already blocks CR/LF from
        // entering via `set_cookies`, but `set_pairs` takes a raw
        // `&str` from a browser `Cookie:` header. If a future axum
        // version relaxed `HeaderMap` validation, an embedded `\r\n`
        // would land in the jar and the next `cookies()` call would
        // build a `HeaderValue` containing control chars,
        // `from_str` would error, and the entire `Cookie:` header
        // would be silently dropped — costing the user their
        // session. Defence-in-depth: reject at insertion.
        jar.set_pairs(["X=foo\r\nInjected: yes", "kept=yes"]);
        assert_eq!(jar.snapshot(), vec![("kept".into(), "yes".into())]);
        let header = jar.cookies(&url()).unwrap();
        assert_eq!(header.to_str().unwrap(), "kept=yes");
    }

    #[test]
    fn rejects_control_chars_in_name() {
        let jar = NameKeyedJar::new();
        jar.set_pairs(["bad\rname=v", "ok=v"]);
        assert_eq!(jar.snapshot(), vec![("ok".into(), "v".into())]);
    }

    #[test]
    fn debug_does_not_leak_cookie_values() {
        let jar = NameKeyedJar::new();
        jar.set_pairs(["JSESSIONID=SECRET-VALUE-DO-NOT-LOG"]);
        let rendered = format!("{jar:?}");
        assert!(
            !rendered.contains("SECRET-VALUE"),
            "Debug must not leak cookie values, got: {rendered}"
        );
        assert!(rendered.contains("JSESSIONID"));
        assert!(rendered.contains("len"));
    }
}
