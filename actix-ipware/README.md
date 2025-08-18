# `actix-ipware`

<!-- prettier-ignore-start -->

[![crates.io](https://img.shields.io/crates/v/actix-ipware?label=latest)](https://crates.io/crates/actix-ipware)
[![Documentation](https://docs.rs/actix-ipware/badge.svg?version=0.1.0)](https://docs.rs/actix-ipware/0.1.0)
![Version](https://img.shields.io/badge/rustc-1.72+-ab6000.svg)
![License](https://img.shields.io/crates/l/actix-ipware.svg)
<br />
[![dependency status](https://deps.rs/crate/actix-ipware/0.1.0/status.svg)](https://deps.rs/crate/actix-ipware/0.1.0)
[![Download](https://img.shields.io/crates/d/actix-ipware.svg)](https://crates.io/crates/actix-ipware)

<!-- prettier-ignore-end -->

<!-- cargo-rdme start -->

Actix-Web Middleware to Retrieve the Client's IP Address

Uses a rust reimplementation of the famous python library
[ipware](https://github.com/un33k/python-ipware), to determine
a best-guess for the real-ip of the client.

## Examples

```rust
use actix_web::{App, http::header::HeaderName};
use actix_ipware::{IpWare, Middleware};

let mut ipware = IpWare::default();
ipware
  .proxy_count(Some(1))
  .trust_proxy("1.2.3.4")
  .trust_proxy("5.6.7.8");

let app = App::new()
  .wrap(Middleware::new(ipware));
```

<!-- cargo-rdme end -->
