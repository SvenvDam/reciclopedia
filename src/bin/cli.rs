#[macro_use]
extern crate log;

use std::env;

use structopt::StructOpt;

use reciclopedia::db;
use reciclopedia::models::cli::Args;
use reciclopedia::models::cli::Command::*;
use reciclopedia::repository::UserRepository;

fn main() {
    dotenv::dotenv().ok();

    env_logger::init();

    let args: Args = Args::from_args();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = db::get_conn_pool(db_url);

    match args.cmd {
        AddUser { username, password } => {
            info!("Creating user {}", &username);
            UserRepository::create_user(&pool.get().unwrap(), username, password).unwrap()
        }
        DeleteUser { username } => {
            info!("Deleting user {}", &username);
            UserRepository::delete_user(&pool.get().unwrap(), username).unwrap()
        }
    }
}