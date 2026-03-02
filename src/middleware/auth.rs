use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use std::rc::Rc;

use crate::config::Config;
use crate::repositories::UserRepository;
use crate::utils::ApiError;
use crate::DbPool;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    username: String,
    role: String,
    exp: usize,
}

// Authentication middleware
pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service: Rc::new(service) }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
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
        // Skip auth for non-API routes and login endpoint
        if !req.path().starts_with("/api")
            || req.path() == "/api/auth/login"
            || req.path() == "/api/v1/auth/login"
            || req.path() == "/api/health"
            || req.path() == "/api/ready"
            || req.path() == "/api/version"
            || req.path() == "/api/v1"
            || req.path() == "/api/v1/health"
            || req.path() == "/api/v1/ready"
            || req.path() == "/api/v1/version"
        {
            let service = self.service.clone();
            return Box::pin(async move { service.call(req).await });
        }

        // Extract token from Authorization header
        let auth_header = req.headers().get("Authorization");

        if auth_header.is_none() {
            log::warn!("Authorization header missing for path: {}", req.path());
        }

        let token = match auth_header {
            Some(header_value) => {
                let header_str = header_value.to_str().unwrap_or("");
                if !header_str.is_empty() {
                    log::debug!("Authorization header present for path: {}", req.path());
                }
                header_str.strip_prefix("Bearer ")
            }
            None => None,
        };

        let token = match token {
            Some(t) => t.to_string(),
            None => {
                return Box::pin(async move {
                    Err(actix_web::error::ErrorUnauthorized(ApiError::Unauthorized(
                        "Missing or invalid Authorization header".to_string(),
                    )))
                });
            }
        };

        // Verify JWT token (secret loaded once in app config)
        let app_config = match req.app_data::<web::Data<Config>>() {
            Some(cfg) => cfg.clone(),
            None => {
                return Box::pin(async move {
                    Err(actix_web::error::ErrorInternalServerError(ApiError::InternalServerError(
                        "Server configuration missing".to_string(),
                    )))
                });
            }
        };
        let token_data = match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(app_config.jwt_secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(data) => data,
            Err(_) => {
                return Box::pin(async move {
                    Err(actix_web::error::ErrorUnauthorized(ApiError::Unauthorized(
                        "Invalid token".to_string(),
                    )))
                });
            }
        };

        // Get user from database
        let pool = req
            .app_data::<actix_web::web::Data<DbPool>>()
            .expect("Database pool should be configured in app_data")
            .clone();
        let user_id = token_data.claims.sub.clone();

        let service = self.service.clone();

        Box::pin(async move {
            let mut conn = pool.get().map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!(
                    "Database connection error: {}",
                    e
                ))
            })?;

            let user = UserRepository::find_by_id(&mut conn, &user_id).map_err(|e| {
                log::error!("Auth middleware: DB error for user_id '{}': {}", user_id, e);
                actix_web::error::ErrorUnauthorized(ApiError::Unauthorized(
                    "User not found".to_string(),
                ))
            })?;

            if !user.is_active {
                return Err(actix_web::error::ErrorUnauthorized(ApiError::Unauthorized(
                    "User inactive".to_string(),
                )));
            }

            // Store user in request extensions
            req.extensions_mut().insert(user);

            // Call the next service
            service.call(req).await
        })
    }
}
