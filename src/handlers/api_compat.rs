use actix_web::{http::header, route, web, HttpRequest, HttpResponse};

fn build_legacy_location(req: &HttpRequest, tail: Option<&str>) -> String {
    let mut location = match tail {
        Some(value) if !value.is_empty() => format!("/api/{}", value),
        _ => "/api".to_string(),
    };

    if let Some(query) = req.uri().query() {
        location.push('?');
        location.push_str(query);
    }

    location
}

#[route(
    "/api/v1",
    method = "GET",
    method = "POST",
    method = "PUT",
    method = "PATCH",
    method = "DELETE",
    method = "OPTIONS",
    method = "HEAD"
)]
async fn compat_redirect_root(req: HttpRequest) -> HttpResponse {
    let location = build_legacy_location(&req, None);
    HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, location))
        .finish()
}

#[route(
    "/api/v1/{tail:.*}",
    method = "GET",
    method = "POST",
    method = "PUT",
    method = "PATCH",
    method = "DELETE",
    method = "OPTIONS",
    method = "HEAD"
)]
async fn compat_redirect(req: HttpRequest, tail: web::Path<String>) -> HttpResponse {
    let location = build_legacy_location(&req, Some(&tail.into_inner()));
    HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, location))
        .finish()
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(compat_redirect_root).service(compat_redirect);
}
