use crate::state::app_state::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn auth_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get("this form auth "))
        .with_state(state)
}
