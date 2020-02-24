#[macro_use]
extern crate juniper;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel;

pub mod db;
pub mod models;
pub mod route;
pub mod graphql;
pub mod schema;