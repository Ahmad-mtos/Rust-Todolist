use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::{models::QueryTask, schema::tasks};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn fetch_all(connection: &mut SqliteConnection) -> Vec<QueryTask> {
    let results = tasks::table
        .select(QueryTask::as_select())
        .load(connection)
        .expect("Error loading table");
    return results;
}

pub fn add_task(connection: &mut SqliteConnection, new_task: QueryTask) -> bool {
    if let Err(_) = diesel::insert_into(tasks::table)
        .values(&new_task)
        .returning(QueryTask::as_returning())
        .get_result(connection) {
        false
    } else {
        true
    }
}