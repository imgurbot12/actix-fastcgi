use std::rc::Rc;
use std::{ops::Deref, time::Duration};

use actix_web::{
    Error as ActixError, ResponseError,
    body::BoxBody,
    dev::{Service, ServiceRequest, ServiceResponse, forward_ready},
    rt::time::sleep,
};
use derive_more::{Display, Error, From};
use futures_core::future::LocalBoxFuture;

use crate::futures::{Either, select};

/// Errors which occur when processing the inner service
#[derive(Debug, Display, From, Error)]
pub enum TimeoutError {
    /// Service error
    Service(actix_web::Error),
    /// Service call timeout
    #[display("Operation timeout")]
    Timeout,
}

impl ResponseError for TimeoutError {
    /// Returns `500 Internal Server Error`.
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

/// Assembled Timeout service
#[derive(Clone)]
pub struct TimeoutService<S>(pub(crate) Rc<TimeoutInner<S>>);

impl<S> Deref for TimeoutService<S> {
    type Target = TimeoutInner<S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct TimeoutInner<S> {
    pub(crate) timeout: Duration,
    pub(crate) service: Rc<S>,
}

impl<S> Service<ServiceRequest> for TimeoutService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let this = Rc::clone(&self.0);
        Box::pin(async move {
            Ok(
                match select(sleep(this.timeout), this.service.call(req)).await {
                    Either::Left(_) => Err(TimeoutError::Timeout)?,
                    Either::Right(res) => res.map_err(TimeoutError::Service)?,
                },
            )
        })
    }
}
