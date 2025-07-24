//use crate::handlers::user::user_handler;
use crate::state::app_state::AppState;
use axum::{
    //routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        // .route("/user/:id",get(user_handler))
        // .route("/user", post(user_handler))
        .with_state(state)
}
