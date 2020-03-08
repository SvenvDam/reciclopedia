use juniper::{FieldResult, RootNode};

use crate::db::Context;
use crate::models::graphql::Recipe;
use crate::repository::RecipeRepository;

pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn get_recipe_by_name(ctx: &Context, name: String) -> FieldResult<Option<Recipe>> {
        RecipeRepository::get_recipe_by_name(&ctx.pool.get().unwrap(), &name)
    }
}

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn test(str: String) -> FieldResult<String> {
        Ok(str)
    }
}

pub fn schema() -> RootNode<'static, Query, Mutation> {
    juniper::RootNode::new(Query, Mutation)
}