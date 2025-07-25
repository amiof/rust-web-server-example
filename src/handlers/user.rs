use axum::{
    extract::{Path, Query},
    Json,
};
use serde::{Deserialize, Serialize};

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
