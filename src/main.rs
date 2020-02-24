use recipes_graphql::db;
use recipes_graphql::route;
use dotenv;

fn main() {
    dotenv::dotenv().ok();

    simple_logger::init_with_level(log::Level::Info).unwrap();

    let pool = db::get_conn_pool();

    warp::serve(route::get_routes(pool))
        .run(([0, 0, 0, 0], 8080))
}
