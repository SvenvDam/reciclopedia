use diesel::PgConnection;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult, Value};

use crate::db::PostgresPool;
use crate::models::{graphql as gql, postgres as pg};
use crate::schema::*;

type PgResult<T> = Result<T, diesel::result::Error>;

fn as_field_result<T>(pg_result: PgResult<T>) -> FieldResult<T> {
    pg_result.map_err(|e| FieldError::from(e))
}

pub struct RecipeRepository {
    pub pool: PostgresPool
}

impl RecipeRepository {
    pub fn get_recipe_by_name(
        conn: &PgConnection,
        recipe_name: &str,
    ) -> FieldResult<Option<gql::Recipe>> {
        let recipe = as_field_result(
            recipes::table
                .filter(recipes::name.eq(recipe_name))
                .get_result::<pg::Recipe>(conn)
                .optional()
        )?;

        match recipe {
            Some(r) => as_field_result(
                pg::RecipeIngredient::belonging_to(&r)
                    .inner_join(ingredients::table)
                    .get_results::<(pg::RecipeIngredient, pg::Ingredient)>(conn)
            ).map(|ings| {
                Some(gql::Recipe::from_pg(&r, &ings))
            }),
            None => Err(FieldError::new(format!("No recipe with name {}", recipe_name), Value::null()))
        }
    }

    pub fn get_recipes_by_ingredient_name(
        conn: &PgConnection,
        ingredient_name: &str,
    ) -> FieldResult<Vec<gql::Recipe>> {
        let pg_result = as_field_result(
            ingredients::table
                .filter(ingredients::name.eq(ingredient_name))
                .get_result::<pg::Ingredient>(conn)
                .optional()
        )?;

        let pg_recipes = match pg_result {
            Some(ing) => as_field_result(
                pg::RecipeIngredient::belonging_to(&ing)
                    .inner_join(recipes::table)
                    .select(recipes::all_columns)
                    .get_results::<pg::Recipe>(conn)
            )?,
            None => return Err(FieldError::new(
                format!("No ingredient with name {}", ingredient_name),
                Value::null(),
            ))
        };

        let pg_recipes_with_ingredients = pg::RecipeIngredient::belonging_to(&pg_recipes)
            .inner_join(ingredients::table)
            .get_results::<(pg::RecipeIngredient, pg::Ingredient)>(conn)?
            .grouped_by(&pg_recipes);

        let found_recipes = pg_recipes
            .iter()
            .zip(pg_recipes_with_ingredients)
            .map(|(r, ings)| gql::Recipe::from_pg(&r, &ings))
            .collect();

        Ok(found_recipes)
    }

    pub fn get_recipes_by_ingredient_names(
        conn: &PgConnection,
        ingredient_names: &Vec<String>,
    ) -> FieldResult<Vec<gql::Recipe>> {
        let pg_ingredients: Vec<pg::Ingredient> = as_field_result(
            ingredients::table
                .filter(ingredients::name.eq_any(ingredient_names))
                .get_results::<pg::Ingredient>(conn)
        )?;

        if ingredient_names.len() != pg_ingredients.len() {
            return Err(FieldError::new(
                format!("Not all ingredients found. Wanted: {:?}. Found: {:?}", ingredient_names, pg_ingredients),
                Value::null(),
            ));
        }

        let pg_recipes: Vec<pg::Recipe> = as_field_result(pg::RecipeIngredient::belonging_to(&pg_ingredients)
            .inner_join(recipes::table)
            .select(recipes::all_columns)
            .get_results::<pg::Recipe>(conn)
        )?;

        let pg_recipes_with_ingredients: Vec<(Vec<(pg::RecipeIngredient, pg::Ingredient)>, pg::Recipe)> =
            pg::RecipeIngredient::belonging_to(&pg_recipes)
                .inner_join(ingredients::table)
                .get_results::<(pg::RecipeIngredient, pg::Ingredient)>(conn)?
                .grouped_by(&pg_recipes)
                .into_iter()
                .zip(pg_recipes)
                .filter(|(ings, _)| {
                    let found: Vec<&pg::Ingredient> = ings.into_iter().map(|(_, i)| i).collect();
                    pg_ingredients.iter().all(|ing| found.contains(&ing))
                })
                .collect();

        Ok(
            pg_recipes_with_ingredients
                .iter()
                .map(|(ings, r)| gql::Recipe::from_pg(&r, &ings))
                .collect()
        )
    }
}