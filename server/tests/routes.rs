use warp::test;

use reciclopedia::repository::UserRepository;
use reciclopedia::routes::get_routes;

#[macro_use]
mod common;

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
    let routes = get_routes(pool);

    let reply = test::request()
        .method("POST")
        .path("/login")
        .body("username=user&password=pwd")
        .reply(&routes);

    assert!(reply.headers().get("Set-Cookie").is_none());

    assert_eq!(reply.status(), 401);
    assert_eq!(reply.body(), "User not found");
}

#[test]
fn test_login_invalid_password() {
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
        .body("username=user&password=INVALID")
        .reply(&routes);

    assert!(reply.headers().get("Set-Cookie").is_none());

    assert_eq!(reply.status(), 401);
    assert_eq!(reply.body(), "Incorrect password");
}

