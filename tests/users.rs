use diesel::PgConnection;
use diesel::prelude::*;

use recipes_graphql::models::postgres::User;
use recipes_graphql::repository::UserRepository;
use recipes_graphql::schema::users::dsl::*;

#[macro_use]
mod common;

#[test]
fn insert_user() {
    setup_pg_test!(conn);

    UserRepository::create_user(
        conn,
        "testuser".into(),
        "password123".into(),
    ).expect("Creating user failed!");

    let mut created_users = users.load::<User>(conn).unwrap();

    assert_eq!(created_users.len(), 1);
    let created_user = created_users.pop().unwrap();

    assert_eq!(created_user.username, "testuser")
}

#[test]
fn delete_user() {
    setup_pg_test!(conn);

    diesel::insert_into(users)
        .values(
            User {
                username: "user".into(),
                salt: "salt".into(),
                hashpwd: "hash".into(),
                token: None,
            }
        )
        .execute(conn)
        .expect("Creating user failed");

    let initial_users = users.load::<User>(conn).unwrap();

    assert_eq!(initial_users.len(), 1);

    UserRepository::delete_user(conn, "user".into()).expect("Deleting user failed");

    let remaining_users = users.load::<User>(conn).unwrap();

    assert_eq!(remaining_users.len(), 0);
}

#[test]
fn login_user() {
    setup_pg_test!(conn);
    UserRepository::create_user(conn, "user".into(), "pwd".into()).unwrap();

    let session_token = UserRepository::try_login(conn, "user", "pwd").unwrap();

    let user = users.find("user").get_result::<User>(conn).unwrap();

    assert_eq!(session_token, user.token.unwrap());

    let invalid_login = UserRepository::try_login(conn, "user", "wrong");
    assert!(invalid_login.is_err());
}

#[test]
fn validate_token() {
    setup_pg_test!(conn);

    diesel::insert_into(users)
        .values(
            User {
                username: "user".into(),
                hashpwd: "hash".into(),
                salt: "salt".into(),
                token: Some("token123".into()),
            }
        )
        .execute(conn)
        .unwrap();

    assert!(UserRepository::validate_token(conn, "user", "token123"));
    assert!(!UserRepository::validate_token(conn, "user", "invalid"));
}