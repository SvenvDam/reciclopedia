use diesel::PgConnection;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

use crate::db::PostgresPool;
use crate::models::{graphql, postgres};
use crate::schema::recipes::dsl as rdsl;

type PgResult<T> = Result<T, diesel::result::Error>;

fn as_fieldresult<T>(pg_result: PgResult<T>) -> FieldResult<T> {
    pg_result.map_err(|e| FieldError::from(e))
}

pub struct RecipeRepository {
    pub pool: PostgresPool
}

impl RecipeRepository {
    pub fn get_recipe_by_name(conn: &PgConnection, recipe_name: &str) -> FieldResult<Option<graphql::Recipe>> {
        let pg_recipe = as_fieldresult(
            rdsl::recipes
                .filter(rdsl::name.eq(recipe_name))
                .get_result::<postgres::Recipe>(conn)
                .optional()
        )?;

        match pg_recipe {
            Some(r) => as_fieldresult(
                postgres::Ingredient::belonging_to(&r).load::<postgres::Ingredient>(conn)
            ).map(|ings| {
                Some(graphql::Recipe::from_pg(&r, &ings))
            }),
            None => Ok(None)
        }
    }
}