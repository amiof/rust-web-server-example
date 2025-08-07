use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::prelude::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(primary_key(uuid))]
pub struct Users {
    pub uuid: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
}
