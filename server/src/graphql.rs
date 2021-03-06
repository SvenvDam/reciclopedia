use juniper::{FieldResult, RootNode};

use crate::db::Context;
use crate::models::graphql::{NewRecipe, Recipe};
use crate::handlers::recipe::*;

pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn recipe_by_name(ctx: &Context, name: String) -> FieldResult<Recipe> {
        recipe_by_name(ctx, name.to_lowercase())
    }

    fn recipes_by_ingredient(ctx: &Context, name: String) -> FieldResult<Vec<Recipe>> {
        recipes_by_ingredient(ctx, name.to_lowercase())
    }

    fn recipes_by_ingredients(ctx: &Context, names: Vec<String>) -> FieldResult<Vec<Recipe>> {
        recipes_by_ingredients(
            ctx,
            names.iter().map(|s| s.to_lowercase()).collect()
        )
    }
}

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn create_recipe(ctx: &Context, recipe: NewRecipe) -> FieldResult<Recipe> {
        create_recipe(ctx, recipe.to_lowercase())
    }

    fn delete_recipe(ctx: &Context, name: String) -> FieldResult<String> {
        delete_recipe(ctx, name.to_lowercase())
    }
}

pub fn schema() -> RootNode<'static, Query, Mutation> {
    juniper::RootNode::new(Query, Mutation)
}
