// use serde::Deserialize;
use diesel::prelude::*;
use diesel::Insertable;
use serde::{Serialize, Deserialize};
use crate::schema::events;

#[derive(Queryable, Identifiable, Selectable, Associations, Serialize)]
#[diesel(belongs_to(Type, foreign_key = type_))]
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

#[derive(Serialize)]
pub struct EventWithRelations {
    pub event: Event,
    pub event_type: Type,
    pub subtypes: Vec<Subtype>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub type_: i32,
    pub description: String,
    pub body: String,
    pub scale: i32,
    pub source: String,
    pub transpired: Option<String>,
    pub published_date: String,
}

#[derive(Identifiable, Associations, Queryable, Selectable)]
#[diesel(belongs_to(Event))]
#[diesel(belongs_to(Subtype))]
#[diesel(table_name = crate::schema::events_subtypes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct EventSubtype {
    pub id: Option<i32>,
    pub event_id: i32,
    pub subtype_id: i32,
}

#[derive(Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::subtypes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Subtype {
    pub id: Option<i32>,
    pub description: String,
}

#[derive(Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::types)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Type {
    pub id: Option<i32>,
    pub description: String,
}
