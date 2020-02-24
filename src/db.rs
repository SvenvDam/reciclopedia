use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::{Pool, PooledConnection};
use std::env;

pub struct Context {
    pub pool: PostgresPool
}

impl juniper::Context for Context {}

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_conn_pool() -> PostgresPool {
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let mgr = ConnectionManager::<PgConnection>::new(db_url);

    r2d2::Pool::builder()
        .build(mgr)
        .expect("could not build connection pool")
}