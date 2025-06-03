use diesel::prelude::*;
use crate::model::Subtype;
use crate::schema::subtypes;

pub fn get_all_subtypes(conn: &mut SqliteConnection) -> QueryResult<Vec<Subtype>> {
    subtypes::table
        .select(Subtype::as_select())
        .load::<Subtype>(conn)
}
