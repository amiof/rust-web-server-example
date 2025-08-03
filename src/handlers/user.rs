use axum::{
    extract::{Path, Query, Request},
    http::{header, HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension, Json,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
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

//pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
//    let token = req
//        .headers()
//        .get(header::AUTHORIZATION)
//        .and_then(|auth_header| auth_header.to_str().ok())
//        .and_then(|auth_value| {
//            if auth_value.starts_with("Bearer") {
//                Some(auth_value[7..].to_owned())
//            } else {
//                None
//            }
//        });
//    let token = if let Some(token) = token {
//        token
//    } else {
//        return Err(StatusCode::UNAUTHORIZED);
//    };
//    let secret_key = "test secret key";
//    let decoding_key = DecodingKey::from_secret(secret_key.as_ref());
//    match decode(&token, &decoding_key, &Validation::default()) {
//        Ok(token_data) => Ok(next.run(req).await),
//        Err(_) => Err(StatusCode::UNAUTHORIZED),
//    }
//}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct Claims {
    sub: String, // Subject (e.g., user ID)
    exp: usize,  // Expiration time
    iat: usize,  // Issued at time
    username: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct TokenValue {
    token: String,
    token2: String,
}

pub async fn create_jwt(
    Path((username, user_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    let expiration = Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: expiration + 3600,
        iat: expiration,
    };

    let secret_key = "rustHello";
    let key = EncodingKey::from_secret(secret_key.as_ref());

    let token = encode(&Header::default(), &claims, &key);
    match token {
        Ok(token_value) => Ok((
            StatusCode::CREATED,
            Json(TokenValue {
                token: token_value.to_string(),
                token2: token_value.to_string(),
            }),
        )
            .into_response()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
