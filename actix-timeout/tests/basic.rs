use std::time::Duration;

use actix_timeout::Timeout;
use actix_web::{
    body::{self, BoxBody},
    dev::ServiceResponse,
    http::StatusCode,
    test::{self, TestRequest},
    web::Path,
};

/// Convert `ServiceResponse` into body content string
pub async fn get_body(res: ServiceResponse<BoxBody>) -> String {
    let content = res.into_body();
    let data = body::to_bytes(content).await.expect("missing body");
    std::str::from_utf8(&data)
        .expect("invalid body")
        .to_string()
}

#[actix_web::get("/{wait}")]
async fn index(path: Path<u64>) -> &'static str {
    let wait = path.into_inner();
    actix_web::rt::time::sleep(Duration::from_millis(wait)).await;
    "Hello World!"
}

#[actix_web::test]
async fn test_timeout() {
    let mw = Timeout::from_millis(300);
    let srv = test::init_service(actix_web::App::new().service(index).wrap(mw)).await;

    let req = TestRequest::with_uri("/100").to_request();
    let res = test::call_service(&srv, req).await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(get_body(res).await, "Hello World!");

    let req = TestRequest::with_uri("/301").to_request();
    let res = test::try_call_service(&srv, req).await;
    assert!(res.is_err());
}
