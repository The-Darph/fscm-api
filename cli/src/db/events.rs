use self::models::*;
use diesel::prelude::*;
use self::schema::events::dsl::*;

pub fn get_all_events(results_limit: Option<String>) -> QueryResult<Event> {
    use self::schema::events::dsl::*;

    let limit: i32 = results_limit.parse().unwrap_or(1000);

    let results = events
        .limit(limit)
        .select(Event::as_select())
        .load(connection)
        .expect("Error loading events");
}
