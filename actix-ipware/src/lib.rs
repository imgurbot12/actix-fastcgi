//! Actix-Web Middleware to Retrieve the Client's IP Address
//!
//! Uses a rust reimplementation of the famous python library
//! [ipware](https://github.com/un33k/python-ipware), to determine
//! a best-guess for the real-ip of the client.
//!
//! Internally, by default, it overwrites the original peer-address
//! assigned to the [`actix_web::HttpRequest`] which allows any
//! future service to easily reference the correct client-ip.
//! However, this is not the only way the ipware service can
//! operate. See [`Behavior`] for more details.
//!
//! # Example
//!
//! ```
//! use actix_web::{App, http::header::HeaderName};
//! use actix_ipware::{IpWare, Middleware};
//!
//! let mut ipware = IpWare::default();
//! ipware
//!     .proxy_count(Some(1))
//!     .trust_proxy("1.2.3.4")
//!     .trust_proxy("5.6.7.8");
//!
//! let app = App::new()
//!   .wrap(Middleware::new(ipware));
//! ```

mod factory;
mod service;

pub use factory::Middleware;
pub use service::{Behavior, IpwareService, PeerAddr};

pub use ipware::{IpWare, IpWareConfig, IpWareProxy};

