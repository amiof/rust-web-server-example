use crate::handlers::{user_body_json_handler, user_handler};
use crate::state::app_state::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(user_handler))
        .route("/body", post(user_body_json_handler))
        .with_state(state)
}
