//! A video, a sound or a download answers a `Range` request with the part
//! it asks for, as Safari needs before it plays a video at all
//! (RFC 9110 section 14).

use axum::body::Body;
use axum::http::header::{ACCEPT_RANGES, CONTENT_LENGTH, CONTENT_RANGE, CONTENT_TYPE, ETAG};
use axum::http::{Method, Request};
use http_body_util::BodyExt;
use khalid_server::{MediaType, Page, PagePath, Pages, SecurityHeaders, router};
use tower::ServiceExt;

const VIDEO: &str = "/blog/site-tour/site-tour.mp4";

/// Ten bytes, so each part is easy to read.
const BYTES: &[u8] = b"0123456789";

/// The status, the named headers in order (empty when missing) and the body.
#[cfg(test)]
async fn ask(method: Method, path: &str, headers: &[(&str, &str)]) -> (u16, Vec<String>, Vec<u8>) {
    let video = Page::new(PagePath::try_new(VIDEO).unwrap(), MediaType::Mp4, BYTES);
    let post = PagePath::try_new("/blog/site-tour/").unwrap();
    let html = Page::new(post, MediaType::Html, "<p>A tour of the site</p>");
    let pages = Pages::new(vec![video, html], "").unwrap();
    let mut request = Request::builder().method(method).uri(path);
    for (name, value) in headers {
        request = request.header(*name, *value);
    }
    let response = router(pages, SecurityHeaders::default())
        .oneshot(request.body(Body::empty()).unwrap())
        .await
        .unwrap();
    let named = [
        CONTENT_RANGE,
        CONTENT_LENGTH,
        ACCEPT_RANGES,
        CONTENT_TYPE,
        ETAG,
    ]
    .map(|name| match response.headers().get(name) {
        Some(value) => value.to_str().unwrap().to_owned(),
        None => String::new(),
    })
    .to_vec();
    let status = response.status().as_u16();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    (status, named, body.to_vec())
}

#[tokio::test]
async fn a_video_answers_a_range_with_that_part() {
    for (range, part, sent) in [
        ("bytes=2-5", "bytes 2-5/10", b"2345".as_slice()),
        ("bytes=7-", "bytes 7-9/10", b"789"),
        ("bytes=-3", "bytes 7-9/10", b"789"),
        ("bytes=-30", "bytes 0-9/10", BYTES),
        ("bytes=8-99", "bytes 8-9/10", b"89"),
        ("bytes=0-0", "bytes 0-0/10", b"0"),
    ] {
        let (status, headers, body) = ask(Method::GET, VIDEO, &[("range", range)]).await;
        assert_eq!((status, body.as_slice()), (206, sent), "{range}");
        assert_eq!(headers[0], part, "{range}");
        assert_eq!(headers[1], sent.len().to_string(), "{range}");
        assert_eq!(headers[2..4], ["bytes", "video/mp4"], "{range}");
        assert!(!headers[4].is_empty(), "{range}");
    }
}

#[tokio::test]
async fn a_range_past_the_end_gets_416_with_the_length() {
    for range in ["bytes=10-", "bytes=10-12", "bytes=-0"] {
        let (status, headers, body) = ask(Method::GET, VIDEO, &[("range", range)]).await;
        assert_eq!(
            (status, headers[0].as_str()),
            (416, "bytes */10"),
            "{range}"
        );
        assert!(body.is_empty(), "{range}");
    }
}

/// No range, several, one it cannot read, a range on `HEAD`, or an
/// `If-Range` with an old tag: the whole file, which still says it takes
/// ranges.
#[tokio::test]
async fn a_range_it_does_not_answer_in_part_gets_the_whole_file() {
    let get = |range| [("range", range)];
    let old = [("range", "bytes=0-1"), ("if-range", "\"old\"")];
    for (method, headers) in [
        (Method::GET, &[][..]),
        (Method::GET, &get("bytes=0-1, 4-5")[..]),
        (Method::GET, &get("bytes=5-2")[..]),
        (Method::GET, &get("items=0-1")[..]),
        (Method::GET, &get("bytes=a-b")[..]),
        (Method::GET, &old[..]),
        (Method::HEAD, &get("bytes=0-1")[..]),
    ] {
        let (status, named, body) = ask(method.clone(), VIDEO, headers).await;
        assert_eq!((status, named[0].as_str()), (200, ""), "{headers:?}");
        assert_eq!(named[1..3], ["10", "bytes"], "{headers:?}");
        if method == Method::GET {
            assert_eq!(body, BYTES, "{headers:?}");
        }
    }
}

#[tokio::test]
async fn an_if_range_with_the_current_tag_gets_the_part() {
    let tag = ask(Method::GET, VIDEO, &[]).await.1[4].clone();
    let asked = [("range", "bytes=0-1"), ("if-range", tag.as_str())];
    assert_eq!(ask(Method::GET, VIDEO, &asked).await.2, b"01");
}

#[tokio::test]
async fn a_page_is_never_sent_in_part() {
    let (status, headers, body) =
        ask(Method::GET, "/blog/site-tour/", &[("range", "bytes=0-1")]).await;
    assert_eq!(
        (status, headers[0].as_str(), headers[2].as_str()),
        (200, "", "")
    );
    assert_eq!(body, b"<p>A tour of the site</p>");
}
