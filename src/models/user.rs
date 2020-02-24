use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::users;

#[derive(Debug, Queryable, Identifiable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserChange {
    pub email: Option<String>,
    pub password: Option<String>,
}
