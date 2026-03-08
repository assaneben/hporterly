use std::env;
use std::time::Duration;

use once_cell::sync::Lazy;
use prometheus::{
    Encoder, HistogramOpts, HistogramVec, IntCounterVec, IntGauge, Registry, TextEncoder,
};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

use crate::DbPool;

struct Metrics {
    registry: Registry,
    http_requests_total: IntCounterVec,
    http_request_duration_seconds: HistogramVec,
    db_pool_connections: IntGauge,
    db_pool_idle_connections: IntGauge,
}

static METRICS: Lazy<Metrics> = Lazy::new(|| {
    let registry = Registry::new();

    let http_requests_total = IntCounterVec::new(
        prometheus::Opts::new(
            "hporterly_http_requests_total",
            "Total number of HTTP requests handled by HPorterly",
        ),
        &["method", "path", "status"],
    )
    .expect("failed to create http request counter");

    let http_request_duration_seconds = HistogramVec::new(
        HistogramOpts::new(
            "hporterly_http_request_duration_seconds",
            "HTTP request latency in seconds",
        )
        .buckets(vec![0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0]),
        &["method", "path", "status"],
    )
    .expect("failed to create http request duration histogram");

    let db_pool_connections = IntGauge::new(
        "hporterly_db_pool_connections",
        "Current number of allocated PostgreSQL pool connections",
    )
    .expect("failed to create db pool connections gauge");

    let db_pool_idle_connections = IntGauge::new(
        "hporterly_db_pool_idle_connections",
        "Current number of idle PostgreSQL pool connections",
    )
    .expect("failed to create db pool idle connections gauge");

    registry
        .register(Box::new(http_requests_total.clone()))
        .expect("failed to register http request counter");
    registry
        .register(Box::new(http_request_duration_seconds.clone()))
        .expect("failed to register http request histogram");
    registry
        .register(Box::new(db_pool_connections.clone()))
        .expect("failed to register db pool connections gauge");
    registry
        .register(Box::new(db_pool_idle_connections.clone()))
        .expect("failed to register db pool idle connections gauge");

    Metrics {
        registry,
        http_requests_total,
        http_request_duration_seconds,
        db_pool_connections,
        db_pool_idle_connections,
    }
});

pub fn init_logging(environment: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _ = LogTracer::init();

    let env_filter = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;
    let format = env::var("LOG_FORMAT").unwrap_or_else(|_| {
        if environment.eq_ignore_ascii_case("production") {
            "json".to_string()
        } else {
            "compact".to_string()
        }
    });

    match format.trim().to_ascii_lowercase().as_str() {
        "json" => tracing_subscriber::registry()
            .with(env_filter)
            .with(
                fmt::layer()
                    .json()
                    .flatten_event(true)
                    .with_current_span(true)
                    .with_span_list(true),
            )
            .try_init()?,
        _ => tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt::layer().compact())
            .try_init()?,
    }

    Ok(())
}

pub fn observe_http_request(method: &str, path: &str, status: u16, elapsed: Duration) {
    let status = status.to_string();
    METRICS
        .http_requests_total
        .with_label_values(&[method, path, status.as_str()])
        .inc();
    METRICS
        .http_request_duration_seconds
        .with_label_values(&[method, path, status.as_str()])
        .observe(elapsed.as_secs_f64());
}

pub fn normalize_path_label(route_pattern: Option<&str>, raw_path: &str) -> String {
    if let Some(pattern) = route_pattern {
        let trimmed = pattern.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }

    match raw_path {
        "/" | "/index.html" => "/".to_string(),
        path if path.starts_with("/assets/") => "/assets/*".to_string(),
        path if path.starts_with("/icons/") => "/icons/*".to_string(),
        path if path.starts_with("/fonts/") => "/fonts/*".to_string(),
        path if path.starts_with("/styles/") => "/styles/*".to_string(),
        path if path.starts_with("/modules/") => "/modules/*".to_string(),
        path if path.ends_with(".js")
            || path.ends_with(".css")
            || path.ends_with(".map")
            || path.ends_with(".woff2") =>
        {
            "/static/*".to_string()
        }
        path => path.to_string(),
    }
}

pub fn render_prometheus_metrics(pool: &DbPool) -> Result<String, String> {
    let state = pool.state();
    METRICS
        .db_pool_connections
        .set(i64::from(state.connections as i32));
    METRICS
        .db_pool_idle_connections
        .set(i64::from(state.idle_connections as i32));

    let metric_families = METRICS.registry.gather();
    let mut buffer = Vec::new();
    TextEncoder::new()
        .encode(&metric_families, &mut buffer)
        .map_err(|error| error.to_string())?;
    String::from_utf8(buffer).map_err(|error| error.to_string())
}
