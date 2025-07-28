use crate::handlers::user::{user_header, user_shared_data};
use crate::handlers::{query_path_handel, user_body_json_handler, user_handler, user_path_handle};
use crate::state::app_state::AppState;
use axum::Extension;
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn user_routes(state: Arc<AppState>) -> Router {
    let share_data = SharedData {
        message: "this is a shared data".to_owned(),
    };
    Router::new()
        .route("/", get(user_handler))
        .route("/body", post(user_body_json_handler))
        .route("/path/{id}", get(user_path_handle))
        .route("/query", get(query_path_handel))
        .route("/header", get(user_header))
        .route("/sharedData", get(user_shared_data))
        .with_state(state)
        .layer(Extension(share_data))
}
