use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,Clone)]
pub struct User{
    pub username: String,
    pub password: String,
    pub email: String,
}