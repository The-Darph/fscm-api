// use serde::Serialize;
// use serde::Deserialize;
use diesel::prelude::*;
use diesel::Insertable;
use serde::Serialize;
use crate::schema::events;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Event {
    pub id: Option<i32>,
    pub type_: i32,
    pub description: String,
    pub body: String,
    pub scale: i32,
    pub source: String,
    pub transpired: Option<String>,
    pub published_date: String,
}

#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent<'a> {
    pub type_: i32,
    pub description: &'a str,
    pub body: &'a str,
    pub scale: i32,
    pub source: &'a str,
    pub transpired: Option<&'a str>,
    pub published_date: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::events_subtypes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct EventSubtype {
    pub id: Option<i32>,
    pub event_id: i32,
    pub subtype_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::subtypes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Subtype {
    pub id: Option<i32>,
    pub description: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::types)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Type {
    pub id: Option<i32>,
    pub description: String,
}
