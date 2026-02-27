use std::sync::Arc;

use tokio::sync::RwLock;

use crate::auth::AuthService;
use crate::config::AppConfig;
use crate::db::DbPool;
use crate::models::TransferRequest;

pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db_pool: Option<DbPool>,
    pub transfers: RwLock<Vec<TransferRequest>>,
    pub auth: AuthService,
}

impl AppState {
    pub fn new(config: AppConfig, db_pool: Option<DbPool>) -> Self {
        let cfg = Arc::new(config);
        let auth = AuthService::new(cfg.clone()).expect("auth service initialization");
        Self {
            config: cfg,
            db_pool,
            transfers: RwLock::new(Vec::new()),
            auth,
        }
    }
}
