use juniper::{FieldError, FieldResult, Value};

use crate::db::Context;
use crate::models::graphql::{NewRecipe, Recipe};
use crate::repository::{RecipeRepository, UserRepository};

pub fn recipe_by_name(ctx: &Context, name: String) -> FieldResult<Recipe> {
    RecipeRepository::get_recipe_by_name(&ctx.pool.get().unwrap(), &name)
}

pub fn recipes_by_ingredient(ctx: &Context, name: String) -> FieldResult<Vec<Recipe>> {
    RecipeRepository::get_recipes_by_ingredient_name(&ctx.pool.get().unwrap(), &name)
}

pub fn recipes_by_ingredients(ctx: &Context, names: Vec<String>) -> FieldResult<Vec<Recipe>> {
    RecipeRepository::get_recipes_by_ingredient_names(&ctx.pool.get().unwrap(), &names)
}

pub fn create_recipe(ctx: &Context, recipe: NewRecipe) -> FieldResult<Recipe> {
    authenticate_action(ctx, || RecipeRepository::insert_recipe(&ctx.pool.get().unwrap(), recipe.clone()))
}

fn authenticate_action<T>(ctx: &Context, action: impl Fn() -> FieldResult<T>) -> FieldResult<T> {
    match ctx {
        Context {
            pool,
            username: Some(u),
            session_token: Some(t)
        } if UserRepository::validate_token(&pool.get().unwrap(), &u, &t) => action(),
        _ => Err(FieldError::new("Login required", Value::null()))
    }
}

