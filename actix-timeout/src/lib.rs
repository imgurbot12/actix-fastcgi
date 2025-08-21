//! Actix-Web Response Processing Timeout Middleware
//!
//! # Example
//!
//! ```
//! use actix_web::{App, web::Path};
//! use actix_timeout::Timeout;
//!
//! #[actix_web::get("/{wait}")]
//! async fn potentially_long_process(wait: Path<u64>) -> &'static str {
//!     // lots of work happening here
//!     use std::time::Duration;
//!     let wait = wait.into_inner();
//!     actix_web::rt::time::sleep(Duration::from_millis(wait)).await;
//!
//!     "Hello World!"
//! }
//!
//! let app = App::new()
//!     .wrap(Timeout::from_secs(1))
//!     .service(potentially_long_process);
//! ```

mod factory;
mod futures;
mod service;

pub use factory::Timeout;
pub use service::{TimeoutError, TimeoutService};
