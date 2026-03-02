use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::Uri;
use actix_web::Error;
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::rc::Rc;

pub struct ApiVersionCompatMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ApiVersionCompatMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ApiVersionCompatMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiVersionCompatMiddlewareService { service: Rc::new(service) }))
    }
}

pub struct ApiVersionCompatMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ApiVersionCompatMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut req = req;
        if let Some(rewritten_uri) = rewrite_api_v1_uri(req.uri()) {
            match rewritten_uri.parse::<Uri>() {
                Ok(parsed) => {
                    req.head_mut().uri = parsed;
                }
                Err(error) => {
                    log::warn!("Failed to rewrite API v1 URI '{}': {}", rewritten_uri, error);
                }
            }
        }

        let service = self.service.clone();
        Box::pin(async move { service.call(req).await })
    }
}

fn rewrite_api_v1_uri(uri: &Uri) -> Option<String> {
    let path = uri.path();
    let rewritten_path = if path == "/api/v1" {
        "/api".to_string()
    } else if let Some(rest) = path.strip_prefix("/api/v1/") {
        format!("/api/{}", rest)
    } else {
        return None;
    };

    let mut rewritten = rewritten_path;
    if let Some(query) = uri.query() {
        rewritten.push('?');
        rewritten.push_str(query);
    }

    Some(rewritten)
}
