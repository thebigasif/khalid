//! Part of a file, as a `Range` request asks for it (RFC 9110 section 14).
//! A browser asks for parts to play a video, Safari before it plays one at
//! all, and to go on with a download that stopped.

use std::ops::RangeInclusive;

use axum::body::Body;
use axum::http::{HeaderMap, HeaderValue, Response, StatusCode, header};
use bytes::Bytes;

/// Tells the browser it may ask for part of the file.
pub(crate) const BYTES: HeaderValue = HeaderValue::from_static("bytes");

/// The answer to a `GET` for `body`, a whole file, as the `Range` in
/// `request` asks: 206 with the part, 416 when the part starts past the
/// end, or 200 with all of it. A file is sent whole when the request asks
/// for no part, for several, for one it cannot read, or names in
/// `If-Range` a tag other than `tag`, the file's own. Each answer says it
/// takes ranges and carries its length; the caller adds the type.
pub fn ranged(request: &HeaderMap, body: Bytes, tag: Option<&HeaderValue>) -> Response<Body> {
    let total = body.len();
    let (code, payload, part) = match wanted(request, total, tag) {
        None => (StatusCode::OK, body, None),
        Some(Wanted::Part(part)) if body.get(part.clone()).is_some() => {
            let range = format!("bytes {}-{}/{total}", part.start(), part.end());
            (StatusCode::PARTIAL_CONTENT, body.slice(part), Some(range))
        }
        Some(_) => {
            let range = format!("bytes */{total}");
            (StatusCode::RANGE_NOT_SATISFIABLE, Bytes::new(), Some(range))
        }
    };
    let len = payload.len();
    let mut response = Response::new(Body::from(payload));
    *response.status_mut() = code;
    let headers = response.headers_mut();
    headers.insert(header::ACCEPT_RANGES, BYTES);
    headers.insert(header::CONTENT_LENGTH, HeaderValue::from(len));
    if let Some(value) = part.and_then(|range| HeaderValue::try_from(range).ok()) {
        headers.insert(header::CONTENT_RANGE, value);
    }
    response
}

/// The part of a file a request asks for.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Wanted {
    /// The bytes in this range, both ends inside the file.
    Part(RangeInclusive<usize>),
    /// A part that starts past the end of the file.
    Unsatisfiable,
}

/// The part `request` asks of a file of `total` bytes whose tag is `tag`,
/// or `None` when it gets the whole file.
fn wanted(request: &HeaderMap, total: usize, tag: Option<&HeaderValue>) -> Option<Wanted> {
    let range = request.get(header::RANGE)?.to_str().ok()?;
    let condition = request.get(header::IF_RANGE);
    if condition.is_some_and(|condition| tag != Some(condition)) {
        return None;
    }
    let (first, last) = range.strip_prefix("bytes=")?.trim().split_once('-')?;
    if last.contains(',') {
        return None;
    }
    let Some(end) = total.checked_sub(1) else {
        return Some(Wanted::Unsatisfiable);
    };
    let (start, last) = match (first.trim(), last.trim()) {
        ("", suffix) => match suffix.parse::<usize>().ok()? {
            0 => return Some(Wanted::Unsatisfiable),
            count => (total.saturating_sub(count), end),
        },
        (first, "") => (first.parse().ok()?, end),
        (first, last) => {
            let (start, last): (usize, usize) = (first.parse().ok()?, last.parse().ok()?);
            if last < start {
                return None;
            }
            (start, last.min(end))
        }
    };
    Some(if start > end {
        Wanted::Unsatisfiable
    } else {
        Wanted::Part(start..=last)
    })
}
