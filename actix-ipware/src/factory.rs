use std::future::{Ready, ready};
use std::rc::Rc;

use actix_web::{
    Error,
    body::BoxBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use ipware::IpWare;

use crate::service::{Behavior, IpwareInner, IpwareService};

/// IpWare middleware service
///
/// `Middleware` must be registered with `App::wrap()` method.
///
/// # Examples
///
/// ```
/// use actix_web::{App, http::header::HeaderName};
/// use actix_ipware::{IpWare, Middleware};
///
/// let mut ipware = IpWare::empty();
/// ipware
///     .proxy_count(Some(0))
///     .trust_header(HeaderName::from_static("http_x_forwarded_for"))
///     .trust_header(HeaderName::from_static("x_forwarded_for"));
///
/// let app = App::new().wrap(Middleware::new(ipware));
/// ```
pub struct Middleware {
    ipware: Rc<IpWare>,
    strict: bool,
    behavior: Behavior,
    allow_untrusted: bool,
}

impl Middleware {
    /// Creates a new `Ipware` middleware instance
    #[inline]
    pub fn new(ipware: IpWare) -> Self {
        Self {
            ipware: Rc::new(ipware),
            strict: true,
            behavior: Behavior::Overwrite,
            allow_untrusted: false,
        }
    }

    /// Change behavior of ipware service when applying client ip.
    ///
    /// See [`Behavior`] for more details. Default is [`Behavior::Overwrite`].
    pub fn behavior(mut self, behavior: Behavior) -> Self {
        self.behavior = behavior;
        self
    }

    /// Allow fake/invalid ips in parsed-headers if disabled.
    ///
    /// usage: non-strict mode (X-Forwarded-For: `<fake>, <client>, <proxy1>, <proxy2>`)
    /// The request went through our `<proxy1>` and `<proxy2>`, then our server
    /// We choose the `<client>` ip address to the left our `<proxy1>` and ignore other ips
    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    pub fn allow_untrusted(mut self, allow_untrusted: bool) -> Self {
        self.allow_untrusted = allow_untrusted;
        self
    }
}

impl From<IpWare> for Middleware {
    #[inline]
    fn from(value: IpWare) -> Self {
        Self::new(value)
    }
}

impl<S> Transform<S, ServiceRequest> for Middleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = IpwareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(IpwareService(Rc::new(IpwareInner {
            service: Rc::new(service),
            ipware: Rc::clone(&self.ipware),
            strict: self.strict,
            behavior: self.behavior.clone(),
            allow_untrusted: self.allow_untrusted,
        }))))
    }
}
