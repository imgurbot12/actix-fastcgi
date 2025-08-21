use std::future::{Ready, ready};
use std::rc::Rc;
use std::time::Duration;

use actix_web::{
    Error,
    body::BoxBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};

use crate::service::{TimeoutInner, TimeoutService};

/// Timeout middleware service
///
/// `Middleware` must be registered with `App::wrap()` method.
///
/// # Examples
///
/// ```
/// use actix_web::App;
/// use actix_timeout::Timeout;
///
/// let app = App::new().wrap(Timeout::from_secs(3));
/// ```
pub struct Timeout(Duration);

impl Timeout {
    /// Creates a new timeout middleware instance
    #[inline]
    pub fn new(duration: Duration) -> Self {
        Self(duration)
    }

    /// Creates a timeout middleware with the duration in milliseconds
    #[inline]
    pub fn from_millis(seconds: u64) -> Self {
        Self::new(Duration::from_millis(seconds))
    }

    /// Creates a timeout middleware with the duration in seconds
    #[inline]
    pub fn from_secs(seconds: u64) -> Self {
        Self::new(Duration::from_secs(seconds))
    }
}

impl From<Duration> for Timeout {
    #[inline]
    fn from(value: Duration) -> Self {
        Self::new(value)
    }
}

impl<S> Transform<S, ServiceRequest> for Timeout
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = TimeoutService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TimeoutService(Rc::new(TimeoutInner {
            timeout: self.0.clone(),
            service: Rc::new(service),
        }))))
    }
}
