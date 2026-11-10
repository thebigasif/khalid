//! How often one key may call the API, how many wrong keys one address
//! may send, and how often one address may search the site, before the
//! answer is 429 (RULES.md BL-SEC-5). The numbers come from `[limits]`.

use std::collections::BTreeMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Mutex, PoisonError};
use std::time::Duration;

use axum::extract::{ConnectInfo, Request};
use axum::http::{HeaderMap, HeaderValue, header};
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, TimeDelta, Utc};
use khalid_core::{Clock, LimitSettings, UserMessage};
use utoipa::openapi::header::HeaderBuilder;
use utoipa::openapi::schema::{ObjectBuilder, Type};
use utoipa::openapi::{ContentBuilder, OpenApi, Ref, RefOr, ResponseBuilder};

use crate::keys::KeyId;
use crate::problem::{Problem, ProblemType};

/// The name of the 429 answer every operation behind a key shares in the
/// `OpenAPI` document.
const TOO_MANY: &str = "TooManyRequests";

/// The time each limit is counted over.
const SPAN: TimeDelta = TimeDelta::minutes(1);

/// How many keys and addresses are remembered at most. Past it, the ones
/// whose limit has filled up again are forgotten first, so requests from
/// many addresses cannot grow memory without end.
const MAX_BUCKETS: usize = 50_000;

/// What one limit counts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Bucket {
    /// Wrong keys from one network: an IPv4 address, or an IPv6 /64.
    Refused([u8; 16]),
    /// Requests from one key, or from one staff member's session.
    Key(KeyId),
    /// Searches from one network, counted like wrong keys.
    Search([u8; 16]),
}

/// Whether a look at a bucket takes a request from it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Take {
    /// Only says whether a request is left.
    Look,
    /// Takes the request when one is left.
    Count,
}

/// Which API a request came through, since each has its own limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Api {
    /// The Content API, read by themes and front ends for every reader.
    Content,
    /// The Admin API, which changes the site.
    Admin,
}

/// The request limits both APIs share, kept in memory.
///
/// Each limit allows its whole minute's worth of requests at once and
/// earns one back each sixtieth of the minute's count, so a key that
/// keeps under its rate is never refused. A request is counted against
/// its key only once the key is accepted. A wrong key is counted against
/// the network it came from, and an address with no wrong keys left is
/// refused before its key is checked, since checking an Argon2id hash is
/// slow on purpose. A request with no key costs nothing and is not counted.
#[derive(Debug)]
pub struct ApiLimits {
    clock: Box<dyn Clock>,
    /// How many requests each limit allows a minute.
    per_minute: LimitSettings,
    /// When each bucket is full again. A time already past means full.
    full_at: Mutex<BTreeMap<Bucket, DateTime<Utc>>>,
}

impl ApiLimits {
    /// Limits of `per_minute` that read the time from `clock`.
    pub fn new(clock: impl Clock + 'static, per_minute: LimitSettings) -> Self {
        Self {
            clock: Box::new(clock),
            per_minute,
            full_at: Mutex::new(BTreeMap::new()),
        }
    }

    /// How long `address` must wait before it may send a key again, or
    /// `None` when it may now. Counts nothing.
    pub(crate) fn paused(&self, address: IpAddr) -> Option<Duration> {
        let bucket = Bucket::Refused(network(address));
        self.take(bucket, self.per_minute.wrong_keys_per_minute, Take::Look)
            .err()
    }

    /// Counts a wrong key from `address`.
    pub(crate) fn refused(&self, address: IpAddr) {
        let bucket = Bucket::Refused(network(address));
        // A refusal past the limit only happens when two arrive together;
        // the next one is paused either way.
        let _ = self.take(bucket, self.per_minute.wrong_keys_per_minute, Take::Count);
    }

    /// Counts a search from the reader at `address`, as [`client_of`]
    /// finds it.
    ///
    /// # Errors
    ///
    /// How long the address must wait when it has no searches left; the
    /// refused search is not counted.
    pub fn search(&self, address: IpAddr) -> Result<(), Duration> {
        let bucket = Bucket::Search(network(address));
        self.take(bucket, self.per_minute.search_per_minute, Take::Count)
    }

    /// Counts a request from the accepted `key` through `api`.
    ///
    /// # Errors
    ///
    /// How long the key must wait when it has no requests left; the
    /// refused request is not counted.
    pub(crate) fn admit(&self, key: &KeyId, api: Api) -> Result<(), Duration> {
        let per_minute = match api {
            Api::Content => self.per_minute.content_per_minute,
            Api::Admin => self.per_minute.admin_per_minute,
        };
        self.take(Bucket::Key(key.clone()), per_minute, Take::Count)
    }

    /// Whether `bucket`, which holds `per_minute`, has a request left, and
    /// with [`Take::Count`], takes it.
    fn take(&self, bucket: Bucket, per_minute: u32, how: Take) -> Result<(), Duration> {
        // The settings keep each limit from 1 to 100000.
        let per_minute = i32::try_from(per_minute.max(1)).unwrap_or(i32::MAX);
        let now = self.clock.now();
        let mut full_at = self.full_at.lock().unwrap_or_else(PoisonError::into_inner);
        let from = full_at.get(&bucket).copied().filter(|&at| at > now);
        let next = from.unwrap_or(now) + SPAN / per_minute;
        let wait = next - now - SPAN;
        if wait > TimeDelta::zero() {
            return Err(wait.to_std().unwrap_or_default());
        }
        if how == Take::Count {
            let known = full_at.contains_key(&bucket);
            if full_at.len() >= MAX_BUCKETS && !known {
                full_at.retain(|_, at| *at > now);
            }
            if full_at.len() < MAX_BUCKETS || known {
                full_at.insert(bucket, next);
            }
        }
        Ok(())
    }
}

/// The network `address` is in: itself for IPv4, its /64 for IPv6, which
/// one home or server usually has whole.
fn network(address: IpAddr) -> [u8; 16] {
    match address.to_canonical() {
        IpAddr::V4(v4) => v4.to_ipv6_mapped().octets(),
        IpAddr::V6(v6) => {
            let mut octets = v6.octets();
            octets.iter_mut().skip(8).for_each(|octet| *octet = 0);
            octets
        }
    }
}

/// The address `request` came from: the peer's, or, when the peer is a
/// proxy on this machine or the local network, the last address its
/// `X-Forwarded-For` names. `None` when the server passed no peer, as in
/// tests that call the router directly.
pub(crate) fn client(request: &Request) -> Option<IpAddr> {
    let ConnectInfo(peer) = request.extensions().get::<ConnectInfo<SocketAddr>>()?;
    Some(client_of(peer.ip(), request.headers()))
}

/// The address a request from `peer` with `headers` came from: `peer`, or,
/// when it is a proxy on this machine or the local network, the last
/// address its `X-Forwarded-For` names. The same rule as the comment
/// form's limit in `khalid-comments`.
pub fn client_of(peer: IpAddr, headers: &HeaderMap) -> IpAddr {
    let local = match peer.to_canonical() {
        IpAddr::V4(v4) => v4.is_loopback() || v4.is_private(),
        IpAddr::V6(v6) => v6.is_loopback() || v6.is_unique_local(),
    };
    if !local {
        return peer;
    }
    let forwarded = headers.get_all("x-forwarded-for").into_iter().next_back();
    forwarded
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.rsplit(',').next())
        .and_then(|last| last.trim().parse().ok())
        .unwrap_or(peer)
}

/// The 429 for a key with no requests left for `wait`.
pub(crate) fn busy(wait: Duration) -> Response {
    let seconds = whole_seconds(wait);
    let message = UserMessage::new(
        "Too many requests came from this key in the last minute.",
        "Each key has a limit, so one busy or leaked key cannot slow the site for everyone.",
        format!(
            "Wait {}, then send fewer requests at a time.",
            in_seconds(seconds)
        ),
    );
    too_many(&message, seconds)
}

/// The 429 for an address that sent too many wrong keys, for `wait`.
pub(crate) fn paused(wait: Duration) -> Response {
    let seconds = whole_seconds(wait);
    let message = UserMessage::new(
        "Too many wrong keys came from this address in the last minute.",
        "Checking a key takes the server real work, so the API pauses an address that sends many wrong ones.",
        format!("Check the key, then try again in {}.", in_seconds(seconds)),
    );
    too_many(&message, seconds)
}

fn too_many(message: &UserMessage, seconds: u64) -> Response {
    let mut response = Problem::new(ProblemType::TooManyRequests, message).into_response();
    response
        .headers_mut()
        .insert(header::RETRY_AFTER, HeaderValue::from(seconds));
    response
}

/// `wait` rounded up to whole seconds, at least one, as `Retry-After` takes.
pub fn whole_seconds(wait: Duration) -> u64 {
    let part = u64::from(wait.subsec_nanos() > 0);
    wait.as_secs().saturating_add(part).max(1)
}

/// Lists the 429 answer on every operation of `document` that needs a key,
/// since each key and each address sending wrong keys has a limit. An
/// operation that already lists its own 429 keeps it.
pub(crate) fn document(document: &mut OpenApi) {
    let retry_after = HeaderBuilder::new()
        .schema(Some(ObjectBuilder::new().schema_type(Type::Integer)))
        .description(Some("How many seconds to wait before trying again."))
        .build();
    let problem = ContentBuilder::new()
        .schema(Some(Ref::from_schema_name("Problem")))
        .build();
    let answer = ResponseBuilder::new()
        .description(
            "Too many requests from this key, or too many wrong keys from this address, \
             in the last minute; Retry-After says when to try again",
        )
        .header("Retry-After", retry_after)
        .content("application/problem+json", problem)
        .build();
    let components = document.components.get_or_insert_with(Default::default);
    components
        .responses
        .insert(TOO_MANY.to_owned(), answer.into());
    for item in document.paths.paths.values_mut() {
        let operations = [
            &mut item.get,
            &mut item.put,
            &mut item.post,
            &mut item.patch,
            &mut item.delete,
        ];
        for operation in operations.into_iter().flatten() {
            if operation.security.as_ref().is_none_or(Vec::is_empty) {
                continue;
            }
            let shared = RefOr::Ref(Ref::from_response_name(TOO_MANY));
            let answers = &mut operation.responses.responses;
            answers.entry("429".to_owned()).or_insert(shared);
        }
    }
}

fn in_seconds(seconds: u64) -> String {
    if seconds == 1 {
        "1 second".to_owned()
    } else {
        format!("{seconds} seconds")
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::TimeZone;

    use super::*;

    /// A clock the test moves by hand.
    #[derive(Debug, Clone)]
    struct Hand(Arc<Mutex<DateTime<Utc>>>);

    impl Hand {
        fn start() -> Self {
            let at = Utc.with_ymd_and_hms(2026, 10, 31, 9, 0, 0).unwrap();
            Self(Arc::new(Mutex::new(at)))
        }

        fn wait(&self, by: TimeDelta) {
            *self.0.lock().unwrap() += by;
        }
    }

    impl Clock for Hand {
        fn now(&self) -> DateTime<Utc> {
            *self.0.lock().unwrap()
        }
    }

    fn key() -> KeyId {
        KeyId::try_new("0123456789abcdef").unwrap()
    }

    fn ip(text: &str) -> IpAddr {
        text.parse().unwrap()
    }

    fn usual() -> LimitSettings {
        LimitSettings::default()
    }

    #[test]
    fn a_site_sets_its_own_numbers() {
        let mut own = usual();
        own.admin_per_minute = 2;
        own.wrong_keys_per_minute = 1;
        let limits = ApiLimits::new(Hand::start(), own);
        limits.admit(&key(), Api::Admin).unwrap();
        limits.admit(&key(), Api::Admin).unwrap();
        let wait = limits.admit(&key(), Api::Admin).unwrap_err();
        assert_eq!(wait, Duration::from_secs(30));
        limits.refused(ip("203.0.113.7"));
        assert_eq!(
            limits.paused(ip("203.0.113.7")),
            Some(Duration::from_secs(60))
        );
    }

    #[test]
    fn each_network_has_its_own_searches_apart_from_its_wrong_keys() {
        let clock = Hand::start();
        let limits = ApiLimits::new(clock.clone(), usual());
        let address = ip("2001:db8::1");
        for _ in 0..usual().search_per_minute {
            limits.search(address).unwrap();
        }
        let wait = limits.search(ip("2001:db8::2")).unwrap_err();
        assert_eq!(wait, Duration::from_millis(500));
        assert_eq!(limits.paused(address), None, "searches are not wrong keys");
        limits.search(ip("203.0.113.7")).unwrap();
        clock.wait(TimeDelta::milliseconds(500));
        limits.search(address).unwrap();
    }

    #[test]
    fn a_key_spends_its_minute_at_once_and_earns_one_back_each_tenth_of_a_second() {
        let clock = Hand::start();
        let limits = ApiLimits::new(clock.clone(), usual());
        for _ in 0..usual().admin_per_minute {
            limits.admit(&key(), Api::Admin).unwrap();
        }
        let wait = limits.admit(&key(), Api::Admin).unwrap_err();
        assert_eq!(wait, Duration::from_millis(100));
        clock.wait(TimeDelta::milliseconds(99));
        assert!(
            limits.admit(&key(), Api::Admin).is_err(),
            "refusals are not counted"
        );
        clock.wait(TimeDelta::milliseconds(1));
        limits.admit(&key(), Api::Admin).unwrap();
        assert!(limits.admit(&key(), Api::Admin).is_err());
    }

    #[test]
    fn the_content_api_allows_ten_times_the_admin_api() {
        let limits = ApiLimits::new(Hand::start(), usual());
        for _ in 0..usual().content_per_minute {
            limits.admit(&key(), Api::Content).unwrap();
        }
        let wait = limits.admit(&key(), Api::Content).unwrap_err();
        assert_eq!(wait, Duration::from_millis(10));
    }

    #[test]
    fn an_address_is_paused_after_its_wrong_keys_and_looking_counts_nothing() {
        let clock = Hand::start();
        let limits = ApiLimits::new(clock.clone(), usual());
        let address = ip("203.0.113.7");
        for _ in 0..usual().wrong_keys_per_minute {
            assert_eq!(limits.paused(address), None);
            limits.refused(address);
        }
        assert_eq!(limits.paused(address), Some(Duration::from_secs(6)));
        assert_eq!(limits.paused(ip("203.0.113.8")), None);
        clock.wait(TimeDelta::seconds(6));
        assert_eq!(limits.paused(address), None);
    }

    #[test]
    fn an_ipv6_network_counts_as_one_address() {
        assert_eq!(network(ip("2001:db8::1")), network(ip("2001:db8::ffff")));
        assert_ne!(network(ip("2001:db8::1")), network(ip("2001:db8:0:1::1")));
        assert_eq!(network(ip("::ffff:192.0.2.1")), network(ip("192.0.2.1")));
    }

    #[test]
    fn a_full_memory_forgets_the_buckets_that_filled_up_again() {
        let clock = Hand::start();
        let limits = ApiLimits::new(clock.clone(), usual());
        let full = (0..MAX_BUCKETS).map(|n| {
            let id = KeyId::try_new(&format!("{n:016x}")).unwrap();
            (Bucket::Key(id), clock.now() + TimeDelta::seconds(1))
        });
        limits.full_at.lock().unwrap().extend(full);
        limits.refused(ip("192.0.2.1"));
        assert_eq!(limits.full_at.lock().unwrap().len(), MAX_BUCKETS);
        assert_eq!(limits.paused(ip("192.0.2.1")), None, "no room: not counted");
        clock.wait(TimeDelta::seconds(1));
        limits.refused(ip("192.0.2.1"));
        assert_eq!(limits.full_at.lock().unwrap().len(), 1);
    }

    #[test]
    fn only_a_local_proxy_may_name_the_client() {
        let mut headers = HeaderMap::new();
        let forwarded = HeaderValue::from_static("198.51.100.1, 203.0.113.7");
        headers.insert("x-forwarded-for", forwarded);
        assert_eq!(client_of(ip("127.0.0.1"), &headers), ip("203.0.113.7"));
        assert_eq!(client_of(ip("192.0.2.9"), &headers), ip("192.0.2.9"));
    }

    #[test]
    fn retry_after_rounds_up_to_whole_seconds() {
        assert_eq!(whole_seconds(Duration::from_millis(10)), 1);
        assert_eq!(whole_seconds(Duration::from_millis(6001)), 7);
        assert_eq!(whole_seconds(Duration::from_secs(6)), 6);
    }
}
