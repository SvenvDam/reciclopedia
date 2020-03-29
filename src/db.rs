use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

embed_migrations!();

pub struct Context {
    pub pool: PostgresPool,
    pub username: Option<String>,
    pub session_token: Option<String>
}

impl juniper::Context for Context {}

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_conn_pool() -> PostgresPool {
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let mgr = ConnectionManager::<PgConnection>::new(db_url);

    let pool = r2d2::Pool::builder()
        .max_size(8)
        .build(mgr)
        .expect("could not build connection pool");

    embedded_migrations::run_with_output(
        &pool.get().unwrap(),
        &mut std::io::stdout()
    ).expect("Migrations failed!");

    pool
}