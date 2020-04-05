use std::io::Read;

use warp::test;

use reciclopedia::repository::UserRepository;
use reciclopedia::routes::get_routes;

#[macro_use]
mod common;

#[test]
fn test_index_page() {
    setup_pg_test_pool!(pool);
    let routes = get_routes(pool);

    let reply = test::request()
        .path("/")
        .method("GET")
        .reply(&routes);

    assert_eq!(reply.status(), 200, "OK");

    let mut index_content = String::new();
    std::fs::File::open("./assets/html/index.html")
        .unwrap()
        .read_to_string(&mut index_content)
        .unwrap();

    assert_eq!(
        reply.body(),
        &index_content
    )
}

#[test]
fn test_login_valid_user() {
    setup_pg_test_pool!(pool);
    let routes = get_routes(pool.clone());

    UserRepository::create_user(
        &pool.get().unwrap(),
        "user".into(),
        "pwd".into(),
    ).unwrap();

    let reply = test::request()
        .method("POST")
        .path("/login")
        .body("username=user&password=pwd")
        .reply(&routes);

    let cookie_header = reply
        .headers()
        .get("Set-Cookie")
        .expect("Cookie header not found");

    assert!(
        cookie_header
            .to_str()
            .unwrap()
            .starts_with("User-Session-Token=user##")
    );

    assert_eq!(reply.status(), 301);
}

#[test]
fn test_login_invalid_user() {
    setup_pg_test_pool!(pool);
    let routes = get_routes(pool.clone());

    let reply = test::request()
        .method("POST")
        .path("/login")
        .body("username=user&password=pwd")
        .reply(&routes);

    assert!(reply.headers().get("Set-Cookie").is_none());
}