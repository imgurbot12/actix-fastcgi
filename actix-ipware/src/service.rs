use std::rc::Rc;
use std::{net::SocketAddr, ops::Deref};

use actix_web::HttpMessage;
use actix_web::{
    Error as ActixError,
    body::BoxBody,
    dev::{Service, ServiceRequest, ServiceResponse, forward_ready},
};
use futures_core::future::LocalBoxFuture;
use ipware::IpWare;

/// Unique IP type to apply to [`actix_web::dev::Extensions`]
pub struct PeerAddr(pub Option<SocketAddr>);

/// Behavior Controls for IpWare Middleware.
#[derive(Clone)]
pub enum Behavior {
    /// Overwrite existing peer-address and add original to extensions as [`PeerAddr`]
    Overwrite,
    /// Append Ipware ip-address to extensions as [`PeerAddr`]
    Extension,
}

/// Assembled IpWare service
#[derive(Clone)]
pub struct IpwareService<S>(pub(crate) Rc<IpwareInner<S>>);

impl<S> Deref for IpwareService<S> {
    type Target = IpwareInner<S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct IpwareInner<S> {
    pub(crate) service: Rc<S>,
    pub(crate) ipware: Rc<IpWare>,
    pub(crate) strict: bool,
    pub(crate) behavior: Behavior,
    pub(crate) allow_untrusted: bool,
}

impl<S> Service<ServiceRequest> for IpwareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let this = Rc::clone(&self.0);
        Box::pin(async move {
            let (ip, trusted) = this.ipware.get_client_ip(req.headers(), this.strict);
            let ip = ip.filter(|_| trusted || this.allow_untrusted);

            let peer = req.peer_addr();
            let port = peer.as_ref().map(|addr| addr.port()).unwrap_or_default();
            match this.behavior {
                Behavior::Overwrite => {
                    if let Some(ip) = ip {
                        req.head_mut().peer_addr = Some((ip, port).into());
                    }
                    req.extensions_mut().insert(PeerAddr(peer));
                }
                Behavior::Extension => {
                    let peer = PeerAddr(ip.map(|ip| (ip, port).into()));
                    req.extensions_mut().insert(peer);
                }
            }

            this.service.call(req).await
        })
    }
}
