use crate::model::*;
use crate::model::{NewEvent, Event};
use diesel::prelude::*;
use crate::schema::events::dsl::*;

pub fn get_all_events(
    conn: &mut SqliteConnection,
    results_limit: Option<String>, 
    page: Option<String>
) -> QueryResult<Vec<Event>> {
    use crate::schema::events::dsl::*;

    let limit = parse_optional_limit(results_limit, 1000);
    let offset = parse_optional_limit(page, 0) * limit;

    events
        .limit(limit)
        .offset(offset)
        .select(Event::as_select())
        .load(conn)
}

pub fn insert_event(
    conn: &mut SqliteConnection, 
    new_event: &NewEvent
) -> QueryResult<Event> {
    use crate::schema::events;

    diesel::insert_into(events::table)
        .values(new_event)
        .returning(Event::as_returning())
        .get_result(conn)
}

// This is more of a private function but whatever for now
// Yes, I'm a terrible developer for this.
pub fn parse_optional_limit(input: Option<String>, default: i64) -> i64 {
    input
        .as_deref() // Option<String> -> Option<&str>
        .and_then(|s| s.trim().parse::<i64>().ok())
        .unwrap_or(default)
}
