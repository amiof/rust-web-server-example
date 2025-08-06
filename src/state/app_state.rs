use std::sync::Arc;

use crate::{connect_to_db, DBPool};

pub struct AppState {
    pub message: String,
    pub db_pool: DBPool,
}

pub async fn init_state() -> Arc<AppState> {
    Arc::new(AppState {
        message: "Hello, world! from App State ".to_string(),
        db_pool: connect_to_db().await,
    })
}
