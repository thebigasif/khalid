//! Plain HTTP on port 80 when the site serves HTTPS: the answer to Let's
//! Encrypt's file check (HTTP-01), and a move to HTTPS for everything else.
//! And the domain name those addresses are built from.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use khalid_https::{Certificate, Certificates, Challenges, Domain, port_80, port_80_among};
use tower::ServiceExt;

/// Port 80's answer to `path` for `blog.test`: its status, its
/// `Location`, and its body.
#[cfg(test)]
async fn get(challenges: &Challenges, path: &str) -> (StatusCode, Option<String>, String) {
    let request = Request::get(path).body(Body::empty()).unwrap();
    let domain = Domain::try_new("blog.test").unwrap();
    let response = port_80(&domain, challenges.clone())
        .oneshot(request)
        .await
        .unwrap();
    let status = response.status();
    let location = response
        .headers()
        .get("location")
        .map(|value| value.to_str().unwrap().to_owned());
    let body = response.into_body().collect().await.unwrap().to_bytes();
    (status, location, String::from_utf8(body.to_vec()).unwrap())
}

#[tokio::test]
async fn an_open_check_is_answered_with_its_key_authorization() {
    let challenges = Challenges::new();
    challenges.answer_http("token-1", "token-1.thumbprint");

    let answered = get(&challenges, "/.well-known/acme-challenge/token-1").await;

    assert_eq!(answered.0, StatusCode::OK);
    assert_eq!(answered.2, "token-1.thumbprint");
}

#[tokio::test]
async fn a_check_that_is_not_open_is_not_found() {
    let challenges = Challenges::new();
    challenges.answer_http("token-1", "token-1.thumbprint");
    challenges.clear();

    let answered = get(&challenges, "/.well-known/acme-challenge/token-1").await;

    assert_eq!(answered.0, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn every_other_address_moves_to_https() {
    let answered = get(&Challenges::new(), "/blog/hello/?ref=mail").await;

    assert_eq!(answered.0, StatusCode::PERMANENT_REDIRECT);
    assert_eq!(
        answered.1.as_deref(),
        Some("https://blog.test/blog/hello/?ref=mail")
    );
}

#[tokio::test]
async fn among_many_a_visitor_moves_to_the_name_they_asked_for_once_it_has_a_certificate() {
    let made = rcgen::generate_simple_self_signed(vec!["news.test".to_owned()]).unwrap();
    let (chain, key) = (made.cert.pem(), made.signing_key.serialize_pem());
    let certificates = Certificates::new();
    certificates.set(Certificate::from_pem(chain.as_bytes(), key.as_bytes()).unwrap());
    let home = Domain::try_new("blog.test").unwrap();

    for (host, moved_to) in [
        ("news.test", "https://news.test/a/?b=1"),
        ("NEWS.test.:80", "https://news.test/a/?b=1"),
        ("shop.test", "https://blog.test/a/?b=1"),
        ("203.0.113.7", "https://blog.test/a/?b=1"),
    ] {
        let request = Request::get("/a/?b=1").header("host", host);
        let response = port_80_among(&home, certificates.clone())
            .oneshot(request.body(Body::empty()).unwrap())
            .await
            .unwrap();
        let location = response.headers().get("location").unwrap();

        assert_eq!(response.status(), StatusCode::PERMANENT_REDIRECT, "{host}");
        assert_eq!(location.to_str().unwrap(), moved_to, "{host}");
    }
}

#[test]
fn a_domain_is_kept_in_lower_case_without_its_last_dot() {
    let domain = Domain::try_new("Blog.Example.COM.").unwrap();

    assert_eq!(domain.as_str(), "blog.example.com");
}

#[test]
fn addresses_single_words_and_broken_names_are_not_domains() {
    for text in [
        "",
        "localhost",
        "203.0.113.7",
        "blog example.com",
        "-blog.example.com",
        "blog..example.com",
        "blog.example.com/path",
        "https://blog.example.com",
    ] {
        assert!(Domain::try_new(text).is_err(), "{text:?}");
    }
}
