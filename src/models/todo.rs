use chrono::{DateTime, Utc};
use rocket::serde::Serialize;

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct TodoJson {
    pub id: i32,
    pub title: String,
    pub completed_at: Option<String>,
}
