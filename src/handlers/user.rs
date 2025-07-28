use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    Extension, Json,
};
use serde::{Deserialize, Serialize};

use crate::routes::user::SharedData;

pub async fn user_handler(body: String) -> String {
    body.to_string()
}

#[derive(Serialize, Deserialize)]
pub struct BodyMessage {
    message: String,
}

pub async fn user_body_json_handler(Json(body): Json<BodyMessage>) -> Json<BodyMessage> {
    Json(body)
}

pub async fn user_path_handle(Path(id): Path<String>) -> String {
    id.to_string()
}

pub async fn query_path_handel(Query(query): Query<BodyMessage>) -> String {
    query.message
}

pub async fn user_header(header: HeaderMap) -> String {
    let mut response = String::new();
    if let Some(value) = header.get("User-Agent") {
        response.push_str(&format!("Authorization: {}\n", value.to_str().unwrap()));
    }
    response
}

pub async fn user_shared_data(Extension(share): Extension<SharedData>) -> String {
    share.message
}
