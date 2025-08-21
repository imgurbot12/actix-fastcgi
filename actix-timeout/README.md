# `actix-timeout`

<!-- prettier-ignore-start -->

[![crates.io](https://img.shields.io/crates/v/actix-timeout?label=latest)](https://crates.io/crates/actix-timeout)
[![Documentation](https://docs.rs/actix-timeout/badge.svg?version=0.1.0)](https://docs.rs/actix-timeout/0.1.0)
![Version](https://img.shields.io/badge/rustc-1.72+-ab6000.svg)
![License](https://img.shields.io/crates/l/actix-timeout.svg)
<br />
[![dependency status](https://deps.rs/crate/actix-timeout/0.1.0/status.svg)](https://deps.rs/crate/actix-timeout/0.1.0)
[![Download](https://img.shields.io/crates/d/actix-timeout.svg)](https://crates.io/crates/actix-timeout)

<!-- prettier-ignore-end -->

<!-- cargo-rdme start -->

Actix-Web Response Processing Timeout Middleware

## Examples

```rust
use actix_web::{App, web::Path};
use actix_timeout::Timeout;

#[actix_web::get("/{wait}")]
async fn potentially_long_process(wait: Path<u64>) -> &'static str {
    // lots of work happening here
    use std::time::Duration;
    let wait = wait.into_inner();
    actix_web::rt::time::sleep(Duration::from_millis(wait)).await;

    "Hello World!"
}

let app = App::new()
    .wrap(Timeout::from_secs(1))
    .service(potentially_long_process);
```

<!-- cargo-rdme end -->
