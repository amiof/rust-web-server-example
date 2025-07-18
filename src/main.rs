mod handlers;

use axum::{Router, routing::get};
use axume_project::{handlers::user::user_handler, routes::user::user_routes};

#[tokio::main]
async fn main() {
    // println!("Hello, world!");
    let app = Router::new().route("/", get("Hello, world!"));

    //symotion-prefix) let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    // println!("listening on {}", listener);
    axum::serve(listener, app).await.unwrap();
}
