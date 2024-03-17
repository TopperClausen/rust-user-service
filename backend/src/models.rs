use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub full_name: String,
    pub password_digest: String,
    pub email: String,
    pub created_at: Option<NaiveDateTime>
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub full_name: String,
    pub password_digest: String,
    pub email: String
}
