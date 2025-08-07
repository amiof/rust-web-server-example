use crate::handlers::user::{
    add_user_into_db, auth_middleware, check_auth, check_database, create_jwt, error_status_code,
    get_json, return_201, user_header, user_shared_data,
};
use crate::handlers::{query_path_handel, user_body_json_handler, user_handler, user_path_handle};
use crate::state::app_state::AppState;
use axum::{middleware, Extension};
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
        .layer(Extension(share_data))
        .route("/error_status_code", get(error_status_code))
        .route("/get_json", get(get_json))
        .route("/check_auth", get(check_auth))
        .route_layer(middleware::from_fn(auth_middleware))
        .route("/return_201", get(return_201))
        .route("/get_jwt/{username}/{user_id}", get(create_jwt))
        .route("/check_database", get(check_database))
        .route("/add_user_into_db", post(add_user_into_db))
        .with_state(state)
}
