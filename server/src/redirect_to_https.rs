use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{http, Error, HttpResponse};
use futures::future::{ok, Either, FutureResult};
use futures::Poll;

#[derive(Default, Clone)]
pub struct RedirectHTTPS {
    replace_host: Option<(String, String)>,
}

impl RedirectHTTPS {
    pub fn with_replace_host(s1: &str, s2: &str) -> Self {
        RedirectHTTPS {
            replace_host: Some((s1.to_owned(), s2.to_owned())),
        }
    }
}

impl<S, B> Transform<S> for RedirectHTTPS
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RedirectHTTPSMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RedirectHTTPSMiddleware {
            service,
            replace_host: self.replace_host.clone(),
        })
    }
}

pub struct RedirectHTTPSMiddleware<S> {
    service: S,
    replace_host: Option<(String, String)>,
}

impl<S, B> Service for RedirectHTTPSMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if req.connection_info().scheme() == "https" {
            Either::A(self.service.call(req))
        } else {
            let host = {
                if let Some((s1, s2)) = &self.replace_host {
                    req.connection_info().host().replace(s1, s2)
                } else {
                    req.connection_info().host().to_owned()
                }
            };
            let uri = req.uri().to_owned();
            let url = format!("https://{}{}", host, uri);
            Either::B(ok(req.into_response(
                HttpResponse::MovedPermanently()
                    .header(http::header::LOCATION, url)
                    .finish()
                    .into_body(),
            )))
        }
    }
}
