use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct QueryTask {
    pub id: i32,
    pub done: bool,
    pub title: String,
    pub description: String,
    pub deadline: String,
    pub priority: i32
}