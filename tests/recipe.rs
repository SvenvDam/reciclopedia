use diesel::prelude::*;

use reciclopedia::models::graphql as gql;
use reciclopedia::models::postgres as pg;
use reciclopedia::repository::RecipeRepository;
use reciclopedia::schema::{ingredients, recipe_ingredients, recipes};

#[macro_use]
mod common;

#[test]
fn create_recipe() {
    setup_pg_test_pool!(pool);
    let conn = &pool.get().unwrap();

    RecipeRepository::insert_recipe(
        conn,
        gql::NewRecipe {
            name: "ham sandwich".into(),
            ingredients: vec![
                gql::NewIngredient { name: "bread".into(), qty: Some("2 slices".into()) },
                gql::NewIngredient { name: "ham".into(), qty: None },
            ],
        },
    ).expect("Inserting recipe failed");

    let found_recipes = recipes::table.load::<pg::Recipe>(conn).unwrap();
    assert_eq!(
        found_recipes,
        vec![pg::Recipe { id: 1, name: "ham sandwich".into() }]
    );

    let found_ingredients = ingredients::table.load::<pg::Ingredient>(conn).unwrap();
    assert_eq!(
        found_ingredients,
        vec![
            pg::Ingredient { id: 1, name: "bread".into() },
            pg::Ingredient { id: 2, name: "ham".into() },
        ]
    );

    let found_recipe_ingredients = recipe_ingredients::table.load::<pg::RecipeIngredient>(conn).unwrap();
    assert_eq!(
        found_recipe_ingredients,
        vec![
            pg::RecipeIngredient { recipe_id: 1, ingredient_id: 1, qty: Some("2 slices".into()) },
            pg::RecipeIngredient { recipe_id: 1, ingredient_id: 2, qty: None },
        ]
    );
}

#[test]
fn get_recipe_by_name() {
    setup_pg_test_pool!(pool);
    let conn = &pool.get().unwrap();

    diesel::insert_into(recipes::table)
        .values(pg::Recipe { id: 1, name: "recipe".into() })
        .execute(conn)
        .unwrap();
    diesel::insert_into(ingredients::table)
        .values(pg::Ingredient { id: 1, name: "ingredient".into() })
        .execute(conn)
        .unwrap();
    diesel::insert_into(recipe_ingredients::table)
        .values(pg::RecipeIngredient { recipe_id: 1, ingredient_id: 1, qty: Some("qty".into()) })
        .execute(conn)
        .unwrap();

    let found_recipe = RecipeRepository::get_recipe_by_name(conn, "recipe")
        .expect("Fetching recipe failed");

    assert_eq!(
        found_recipe,
        gql::Recipe {
            name: "recipe".into(),
            ingredients: vec![
                gql::Ingredient { name: "ingredient".into(), qty: Some("qty".into()) }
            ],
        }
    );

    let unfound_recipe = RecipeRepository::get_recipe_by_name(conn, "xxx");

    assert!(unfound_recipe.is_err());
}

#[test]
fn get_recipe_by_ingredient() {
    setup_pg_test_pool!(pool);
    let conn = &pool.get().unwrap();

    diesel::insert_into(recipes::table)
        .values(pg::Recipe { id: 1, name: "recipe1".into() })
        .execute(conn)
        .unwrap();
    diesel::insert_into(recipes::table)
        .values(pg::Recipe { id: 2, name: "recipe2".into() })
        .execute(conn)
        .unwrap();
    diesel::insert_into(recipes::table)
        .values(pg::Recipe { id: 3, name: "recipe3".into() })
        .execute(conn)
        .unwrap();
    diesel::insert_into(ingredients::table)
        .values(pg::Ingredient { id: 1, name: "ingredient".into() })
        .execute(conn)
        .unwrap();
    diesel::insert_into(recipe_ingredients::table)
        .values(pg::RecipeIngredient { recipe_id: 1, ingredient_id: 1, qty: None })
        .execute(conn)
        .unwrap();
    diesel::insert_into(recipe_ingredients::table)
        .values(pg::RecipeIngredient { recipe_id: 2, ingredient_id: 1, qty: None })
        .execute(conn)
        .unwrap();

    let found_recipe = RecipeRepository::get_recipes_by_ingredient_name(conn, "ingredient")
        .expect("Fetching recipes failed");

    assert_eq!(
        found_recipe,
        vec![
            gql::Recipe {
                name: "recipe1".into(),
                ingredients: vec![
                    gql::Ingredient { name: "ingredient".into(), qty: None }
                ],
            },
            gql::Recipe {
                name: "recipe2".into(),
                ingredients: vec![
                    gql::Ingredient { name: "ingredient".into(), qty: None }
                ],
            },
        ]
    );
}

#[test]
fn get_recipe_by_ingredients() {
    setup_pg_test_pool!(pool);
    let conn = &pool.get().unwrap();

    diesel::insert_into(recipes::table)
        .values(pg::Recipe { id: 1, name: "recipe1".into() })
        .execute(conn)
        .unwrap();
    diesel::insert_into(recipes::table)
        .values(pg::Recipe { id: 2, name: "recipe2".into() })
        .execute(conn)
        .unwrap();
    diesel::insert_into(ingredients::table)
        .values(pg::Ingredient { id: 1, name: "ingredient1".into() })
        .execute(conn)
        .unwrap();
    diesel::insert_into(ingredients::table)
        .values(pg::Ingredient { id: 2, name: "ingredient2".into() })
        .execute(conn)
        .unwrap();
    diesel::insert_into(recipe_ingredients::table)
        .values(pg::RecipeIngredient { recipe_id: 1, ingredient_id: 1, qty: None })
        .execute(conn)
        .unwrap();
    diesel::insert_into(recipe_ingredients::table)
        .values(pg::RecipeIngredient { recipe_id: 2, ingredient_id: 1, qty: None })
        .execute(conn)
        .unwrap();
    diesel::insert_into(recipe_ingredients::table)
        .values(pg::RecipeIngredient { recipe_id: 1, ingredient_id: 2, qty: None })
        .execute(conn)
        .unwrap();

    let found_recipe = RecipeRepository::get_recipes_by_ingredient_names(conn, &vec!["ingredient1".into(), "ingredient2".into()])
        .expect("Fetching recipes failed");

    assert_eq!(
        found_recipe,
        vec![
            gql::Recipe {
                name: "recipe1".into(),
                ingredients: vec![
                    gql::Ingredient { name: "ingredient1".into(), qty: None },
                    gql::Ingredient { name: "ingredient2".into(), qty: None },
                ],
            },
        ]
    );
}

#[test]
fn delete_recipe() {
    setup_pg_test_pool!(pool);
    let conn = &pool.get().unwrap();

    diesel::insert_into(recipes::table)
        .values(pg::Recipe { id: 1, name: "recipe1".into() })
        .execute(conn)
        .unwrap();
    diesel::insert_into(recipes::table)
        .values(pg::Recipe { id: 2, name: "recipe2".into() })
        .execute(conn)
        .unwrap();
    diesel::insert_into(ingredients::table)
        .values(pg::Ingredient { id: 1, name: "ingredient1".into() })
        .execute(conn)
        .unwrap();
    diesel::insert_into(ingredients::table)
        .values(pg::Ingredient { id: 2, name: "ingredient2".into() })
        .execute(conn)
        .unwrap();
    diesel::insert_into(recipe_ingredients::table)
        .values(pg::RecipeIngredient { recipe_id: 1, ingredient_id: 1, qty: None })
        .execute(conn)
        .unwrap();
    diesel::insert_into(recipe_ingredients::table)
        .values(pg::RecipeIngredient { recipe_id: 2, ingredient_id: 1, qty: None })
        .execute(conn)
        .unwrap();
    diesel::insert_into(recipe_ingredients::table)
        .values(pg::RecipeIngredient { recipe_id: 1, ingredient_id: 2, qty: None })
        .execute(conn)
        .unwrap();

    RecipeRepository::delete_recipe(conn, "recipe1".into()).expect("Deleting recipe failed!");
    let found_recipes = recipes::table.load::<pg::Recipe>(conn).unwrap();
    assert_eq!(
        found_recipes,
        vec![pg::Recipe { id: 2, name: "recipe2".into() }]
    );

    let found_ingredients = ingredients::table.load::<pg::Ingredient>(conn).unwrap();
    assert_eq!(
        found_ingredients,
        vec![
            pg::Ingredient { id: 1, name: "ingredient1".into() },
        ]
    );

    let found_recipe_ingredients = recipe_ingredients::table.load::<pg::RecipeIngredient>(conn).unwrap();
    assert_eq!(
        found_recipe_ingredients,
        vec![
            pg::RecipeIngredient { recipe_id: 2, ingredient_id: 1, qty: None },
        ]
    );
}
