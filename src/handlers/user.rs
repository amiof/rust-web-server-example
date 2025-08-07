use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Path, Query, Request, State},
    http::{HeaderMap, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use chrono::Utc;
use diesel::{Insertable, PgConnection, RunQueryDsl};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::{models::user::Users, routes::user::SharedData, state::app_state::AppState};

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

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Claims {
    pub sub: String, // Subject (e.g., user ID)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at time
    pub username: String,
}

pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, Response> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });

    let token = if let Some(token) = token {
        dbg!("{ }", &token);
        token
    } else {
        return Err((StatusCode::UNAUTHORIZED, "your auth is not aurhorized".to_string()).into_response());
    };
    let secret_key = "rustHello";
    let decoding_key = DecodingKey::from_secret(secret_key.as_ref());
    match decode::<Claims>(&token, &decoding_key, &Validation::default()) {
        Ok(token_data) => {
            req.extensions_mut().insert(token_data.claims);
            Ok((next.run(req).await).into_response())
        }
        Err(_) => Err((StatusCode::UNAUTHORIZED, "your auth is not aurhorized".to_string()).into_response()),
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct TokenValue {
    token: String,
    token2: String,
}

pub async fn create_jwt(Path((username, user_id)): Path<(String, String)>) -> Result<impl IntoResponse, StatusCode> {
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

pub async fn check_auth(claims: Extension<Claims>) -> Json<Claims> {
    dbg!(&claims.0);
    //format!("this is a auth data = {}", claims.0.sub)
    Json(claims.0)
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

fn get_all_posts(conn: &mut PgConnection) -> Result<Vec<Users>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    users.load::<Users>(conn)
}

pub async fn check_database(State(app_state): State<Arc<AppState>>) -> Result<Json<Vec<Users>>, AppError> {
    let conn = app_state.db_pool.get().await?;
    let result = conn
        .interact(|raw_conn| get_all_posts(raw_conn))
        .await
        .expect("errror")
        .expect("error2");
    Ok(Json(result))
}

//#[derive(Serialize, Deserialize)]
#[derive(Serialize, Deserialize, Insertable,Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct CreateUserFromBody {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

fn create_user(conn: &mut PgConnection, user: &CreateUserFromBody) -> Result<Users, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    diesel::insert_into(users).values(user).get_result::<Users>(conn)
}

pub async fn add_user_into_db(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<CreateUserFromBody>,
) -> Result<Response, StatusCode> {

    let conn = app_state
        .db_pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    dbg!("{}", &body);

    let db_result = conn
        .interact(move |raw_conn| create_user(raw_conn, &body))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match db_result {
        Ok(new_user) => Ok((StatusCode::CREATED, Json(new_user)).into_response()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
