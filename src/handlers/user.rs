use axum::{
    extract::{Path, Query},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
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

pub async fn error_status_code() -> Result<(), StatusCode> {
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn return_201() -> Response {
    (StatusCode::CREATED, ("this is a fake create")).into_response()
}

#[derive(Serialize)]
pub struct UserData {
    message: String,
    id: i32,
    user_name: String,
}

pub async fn get_json() -> Response {
    let data = UserData {
        message: "user Created ".to_string(),
        id: 34,
        user_name: "amiro".to_string(),
    };
    (StatusCode::CREATED, Json(data)).into_response()
}
