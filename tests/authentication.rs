use reciclopedia::db::Context;
use reciclopedia::handlers::recipe::*;
use reciclopedia::models::graphql::{NewRecipe, NewIngredient};
use reciclopedia::schema::recipes::dsl::*;
use diesel::prelude::*;
use reciclopedia::models::postgres::Recipe;
use reciclopedia::repository::UserRepository;

#[macro_use]
mod common;

#[test]
/// the creation of a recipe should not be allowed and executed if no user and token are in the context
fn test_create_recipe_not_authenticated() {
    setup_pg_test_pool!(pool);
    let conn = &pool.get().unwrap();

    let ctx = Context { pool, username: None, session_token: None };

    let recipe = NewRecipe {
        name: "recipe".to_string(),
        ingredients: Vec::<NewIngredient>::new()
    };

    let result = create_recipe(&ctx, recipe);

    assert!(result.is_err());

    let fetched_recipes = recipes.load::<Recipe>(conn).unwrap();

    assert_eq!(fetched_recipes.len(), 0);
}

#[test]
/// the creation of a recipe should not be allowed if an invalid user/token pair is in the context
fn test_create_recipe_invalid_token() {
    setup_pg_test_pool!(pool);
    let conn = &pool.get().unwrap();

    let ctx = Context { pool, username: Some("user".into()), session_token: Some("token".into()) };

    let recipe = NewRecipe {
        name: "recipe".to_string(),
        ingredients: Vec::<NewIngredient>::new()
    };

    let result = create_recipe(&ctx, recipe);

    assert!(result.is_err());

    let fetched_recipes = recipes.load::<Recipe>(conn).unwrap();

    assert_eq!(fetched_recipes.len(), 0);
}

#[test]
/// the creation of a recipe should be allowed if a valid user/token pair is in the context
fn test_create_recipe_authenticated() {
    setup_pg_test_pool!(pool);
    let conn = &pool.get().unwrap();

    UserRepository::create_user(conn, "user".into(), "psw".into()).unwrap();
    let token = UserRepository::try_login(conn, "user", "psw").unwrap();

    let ctx = Context { pool, username: Some("user".into()), session_token: Some(token) };

    let recipe = NewRecipe {
        name: "recipe".to_string(),
        ingredients: Vec::<NewIngredient>::new()
    };

    let result = create_recipe(&ctx, recipe);

    assert!(result.is_ok());

    let fetched_recipes = recipes.load::<Recipe>(conn).unwrap();

    assert_eq!(fetched_recipes.len(), 1);
}