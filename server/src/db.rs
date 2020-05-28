use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

embed_migrations!();

pub struct Context {
    pub pool: PostgresPool,
    pub username: Option<String>,
    pub session_token: Option<String>,
}

impl juniper::Context for Context {}

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_conn_pool(db_url: String) -> PostgresPool {
    let mgr = ConnectionManager::<PgConnection>::new(db_url);

    info!("Creating connection pool...");
    let pool = r2d2::Pool::builder()
        .max_size(8)
        .build(mgr)
        .expect("Error: could not build connection pool");

    let mut migration_logs = Vec::<u8>::new();

    embedded_migrations::run_with_output(
        &pool.get().unwrap(),
        &mut migration_logs,
    ).expect("Error: migrations failed");

    String::from_utf8(migration_logs)
        .expect("Error: Could not read migration logs")
        .lines()
        .for_each(|s| info!("{}", s));

    pool
}