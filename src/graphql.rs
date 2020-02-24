use juniper::{FieldResult, RootNode};

use crate::db::Context;
use crate::models::{Ingredient, Recipe};

pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn test() -> Recipe {
        Recipe {
            id: 0,
            name: "test".into(),
            ingredients: vec![
                Ingredient {
                    id: 0,
                    recipe_id: 0,
                    name: "ing_1".into(),
                    qty: None,
                }
            ]
        }
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