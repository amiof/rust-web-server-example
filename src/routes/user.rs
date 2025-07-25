use crate::handlers::{query_path_handel, user_body_json_handler, user_handler, user_path_handle};
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
        .route("/path/{id}", get(user_path_handle))
        .route("/query", get(query_path_handel))
        .with_state(state)
}
