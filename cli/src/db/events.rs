// use self::models::*;
use self::models::{NewEvent, Event};
use diesel::prelude::*;
use self::schema::events::dsl::*;

pub fn get_all_events(results_limit: Option<String>, page: Option<String>) -> QueryResult<Event> {
    use self::schema::events::dsl::*;

    let limit: i32 = results_limit.parse().unwrap_or(1000);
    // let mut offset: i32 = page.parse().unwrap_or(0);
    let offset:i32 = if page.parse().unwrap_or(0) > 0 {
        page.parse().unwrap_or(0) * limit
    } else {
        0
    }

    let results = events
        .limit(limit)
        .select(Event::as_select())
        .load(connection)
        .expect("Error loading events");
}

pub fn insert_event(conn: &mut SqliteConnection, new_event: &NewEvent) -> QueryResult<usize> {
    use crate::schema::events;

    diesel::insert_into(events::table)
        .values(new_event)
        .returning(Event::as_returning())
        .get_result(conn)
        .expect("Error inserting event into table. events.rs:31")
        // .execute(conn)
}
