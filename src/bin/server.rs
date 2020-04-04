use std::env;

use dotenv;

use reciclopedia::db;
use reciclopedia::route;

fn main() {
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let port: u16 = env::var("PORT")
        .expect("PORT must be set")
        .parse()
        .expect("PORT must be an integer");

    env_logger::init();

    let pool = db::get_conn_pool(db_url);

    warp::serve(route::get_routes(pool))
        .run(([0, 0, 0, 0], port))
}
