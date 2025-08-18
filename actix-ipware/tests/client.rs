//! Basic FastCGI Tests

use std::{net::SocketAddr, str::FromStr};

use actix_ipware::{Behavior, IpWare, Middleware, PeerAddr};
use actix_web::{
    HttpMessage,
    http::header,
    test::{self, TestRequest},
};

#[actix_web::test]
async fn test_overwrite() {
    let mut ipware = IpWare::empty();
    ipware
        .proxy_count(Some(0))
        .trust_header(header::X_FORWARDED_FOR);

    let mw = Middleware::new(ipware);
    let srv = test::init_service(actix_web::App::new().wrap(mw)).await;

    let req = TestRequest::with_uri("/")
        .peer_addr(SocketAddr::from_str("127.0.0.1:8000").unwrap())
        .insert_header((header::X_FORWARDED_FOR, "1.2.3.4"))
        .to_request();
    let res = test::call_service(&srv, req).await;

    let real = res.request().peer_addr().unwrap();
    let ext = res.request().extensions().get::<PeerAddr>().cloned();

    assert_eq!(res.status().to_string(), "404 Not Found");
    assert_eq!(real.to_string(), "1.2.3.4:8000");
    assert_eq!(ext.unwrap().0.to_string(), "127.0.0.1:8000");
}

#[actix_web::test]
async fn test_extension() {
    let mut ipware = IpWare::empty();
    ipware
        .proxy_count(Some(0))
        .trust_header(header::X_FORWARDED_FOR);

    let mw = Middleware::new(ipware).behavior(Behavior::Extension);
    let srv = test::init_service(actix_web::App::new().wrap(mw)).await;

    let req = TestRequest::with_uri("/")
        .peer_addr(SocketAddr::from_str("127.0.0.1:8000").unwrap())
        .insert_header((header::X_FORWARDED_FOR, "1.2.3.4:8000"))
        .to_request();
    let res = test::call_service(&srv, req).await;

    let real = res.request().peer_addr().unwrap();
    let ext = res.request().extensions().get::<PeerAddr>().cloned();

    assert_eq!(res.status().to_string(), "404 Not Found");
    assert_eq!(real.to_string(), "127.0.0.1:8000");
    assert_eq!(ext.unwrap().0.to_string(), "1.2.3.4:8000");
}
