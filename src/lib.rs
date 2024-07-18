pub mod api;
pub mod db;

use api::*;
use db::database::*;
use rocket::http::Status;
use rocket::Build;
use rocket::Rocket;
use std::env;

#[macro_use]
extern crate rocket;

pub fn get_env(key: &str) -> String {
    let not_set = format!("env variable '{}' required", key);
    env::var(key).expect(&not_set)
}

/// Creates a Rocket server with the given database. The database will
/// be shared across all endpoints as a state. The server can be configured
/// by using the `Rocket.toml` file. For more information, see the
/// [Rocket documentation](https://rocket.rs/v0.5/guide/).
pub fn create_server(db: Database) -> Rocket<Build> {
    let utils = routes![get_status, get_version];
    let values = routes![set_value, get_value, delete_value, reset_values];
    let graphs = routes![create_graph, delete_graph, query_graph, reset_graphs];
    rocket::build()
        .manage(db)
        .mount("/", utils)
        .mount("/values", values)
        .mount("/graphs", graphs)
        .register("/", catchers![catch_401, catch_404])
}

// List of default error catchers.

#[catch(404)]
fn catch_404() -> (Status, Response) {
    let message = "Invalid endpoint or method.";
    (Status::NotFound, Response::error(message))
}

#[catch(401)]
fn catch_401() -> (Status, Response) {
    let message = "Invalid x-sahomedb-token header.";
    (Status::Unauthorized, Response::error(message))
}
