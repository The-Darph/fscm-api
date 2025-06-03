use diesel::prelude::*;
use crate::model::Type;
use crate::schema::types;

pub fn get_all_types(conn: &mut SqliteConnection) -> QueryResult<Vec<Type>> {
    types::table
        .select(Type::as_select())
        .load::<Type>(conn)
}
