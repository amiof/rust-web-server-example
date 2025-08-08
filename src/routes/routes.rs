use crate::routes::test::test_routes;
use crate::routes::{auth::auth_routes};
use crate::routes::user::user_routes;
use crate::state::app_state::init_state;
use axum::{http::Method, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

pub async fn app() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/", get("Hello, world!"))
        .nest("/user", user_routes(init_state().await))
        .nest("/auth", auth_routes(init_state().await))
        .nest("/test", test_routes(init_state().await))
        .layer(cors)
}
