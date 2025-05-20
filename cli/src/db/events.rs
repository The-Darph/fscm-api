use crate::model::*;
use crate::model::{NewEvent, Event, Type, Subtype};
use diesel::prelude::*;
use crate::schema::events::dsl::*;
use crate::schema::{events, types, subtypes, events_subtypes};
use csv::ReaderBuilder;
use diesel::sqlite::SqliteConnection;

pub fn get_all_events(
    conn: &mut SqliteConnection,
    results_limit: Option<String>, 
    page: Option<String>,
) -> QueryResult<Vec<EventWithRelations>> {
    let limit = parse_optional_limit(results_limit, 1000);
    let offset = parse_optional_limit(page, 0) * limit;

    // 1. Join events to types
    let event_type_rows = events
        .inner_join(types::table.on(events::type_.nullable().eq(types::id)))
        .limit(limit)
        .offset(offset.into())
        .select((events::all_columns, types::all_columns))
        .load::<(Event, Type)>(conn)?;

    let mut results = Vec::new();

    for (event, event_type) in event_type_rows {
        // 2. Load subtype IDs from join table
        let subtype_ids = events_subtypes::table
            .filter(events_subtypes::event_id.eq(event.id.expect("event.id was None")))
            .select(events_subtypes::subtype_id)
            .load::<i32>(conn)?;

        // 3. Load subtypes
        let related_subtypes = if subtype_ids.is_empty() {
            Vec::new()
        } else {
            subtypes::table
                .filter(subtypes::id.eq_any(subtype_ids))
                .load::<Subtype>(conn)?
        };

        results.push(EventWithRelations {
            event,
            event_type,
            subtypes: related_subtypes,
        });
    }

    Ok(results)
}

pub fn insert_event(
    conn: &mut SqliteConnection, 
    csv_data: &[u8],
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_data);

    let mut inserted_count = 0;

    for result in reader.deserialize::<NewEvent>() {
        let event = result?;
        diesel::insert_into(events::table)
            .values(&event)
            .execute(conn)?;
        inserted_count += 1;
    }

    Ok(inserted_count)
}

// This is more of a private function but whatever for now
// Yes, I'm a terrible developer for this.
pub fn parse_optional_limit(input: Option<String>, default: i64) -> i64 {
    input
        .as_deref() // Option<String> -> Option<&str>
        .and_then(|s| s.trim().parse::<i64>().ok())
        .unwrap_or(default)
}
