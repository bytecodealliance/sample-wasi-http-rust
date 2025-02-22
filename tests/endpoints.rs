mod utils;
use reqwest::blocking::Client;
use std::time::Instant;
use utils::ServerGuard;

#[test]
fn root() {
    let _guard = ServerGuard::new();

    let response = Client::new().get("http://localhost:8080/").send().unwrap();

    assert!(response.status().is_success());
    assert_eq!(
        response.text().unwrap().trim(),
        "Hello, wasi:http/proxy world!"
    );
}

#[test]
fn wait() {
    let _guard = ServerGuard::new();

    let start = Instant::now();
    let response = Client::new()
        .get("http://localhost:8080/wait")
        .send()
        .unwrap();

    assert!(response.status().is_success());
    assert!(
        start.elapsed().as_secs() >= 1,
        "Response returned too quickly"
    );
}

#[test]
fn echo() {
    let _guard = ServerGuard::new();

    let response = Client::new()
        .post("http://localhost:8080/echo")
        .body("Hello, WASI!")
        .send()
        .unwrap();

    assert!(response.status().is_success());
    assert_eq!(response.text().unwrap(), "Hello, WASI!");
}

#[test]
fn echo_headers() {
    let _guard = ServerGuard::new();
    let response = Client::new()
        .get("http://localhost:8080/echo-headers")
        .header("X-Test-Header", "test-value")
        .header("X-Another-Header", "another-value")
        .send()
        .unwrap();

    assert!(response.status().is_success());
    let body = response.text().unwrap();
    assert!(body.contains("x-test-header: test-value"));
    assert!(body.contains("x-another-header: another-value"));
}

#[test]
fn echo_trailers() {
    let _guard = ServerGuard::new();
    let response = Client::new()
        .post("http://localhost:8080/echo-trailers")
        .header("trailer", "x-test-trailer")
        .body("Hello with trailers!")
        .send()
        .unwrap();

    assert!(response.status().is_success());
    // Note: reqwest doesn't directly support HTTP trailers,
    // so we just verify the endpoint responds successfully
}
