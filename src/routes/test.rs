use std::sync::Arc;

use axum::{routing::get, Router};

use crate::state::app_state::AppState;

pub fn test_routes(state: Arc<AppState>) -> Router {
    Router::new().route("/", get("this form test ")).with_state(state)
}
