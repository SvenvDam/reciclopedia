use diesel::pg::upsert::excluded;
use diesel::PgConnection;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult, Value};

use crate::db::PostgresPool;
use crate::models::{graphql as gql, postgres as pg};
use crate::schema::*;

type PgResult<T> = Result<T, diesel::result::Error>;

fn as_field_result<T>(pg_result: PgResult<T>) -> FieldResult<T> {
    pg_result.map_err(FieldError::from)
}

pub struct RecipeRepository {
    pub pool: PostgresPool
}

impl RecipeRepository {
    pub fn get_recipe_by_name(
        conn: &PgConnection,
        recipe_name: &str,
    ) -> FieldResult<gql::Recipe> {
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
                gql::Recipe::from_pg(&r, &ings)
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
        ingredient_names: &[String],
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
                    let found: Vec<&pg::Ingredient> = ings.iter().map(|(_, i)| i).collect();
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

    pub fn insert_recipe(conn: &PgConnection, recipe: gql::NewRecipe) -> FieldResult<gql::Recipe> {
        conn.transaction(|| {
            let inserted_recipe: pg::Recipe = as_field_result(
                diesel::insert_into(recipes::table)
                    .values(pg::NewRecipe::from_graphql(&recipe))
                    .on_conflict(recipes::name)
                    .do_update()
                    .set(recipes::name.eq(excluded(recipes::name)))
                    .get_result(conn)
            )?;

            let inserted_ingredients: Vec<pg::Ingredient> = as_field_result(
                diesel::insert_into(ingredients::table)
                    .values(pg::NewIngredient::from_graphql_many(&recipe.ingredients))
                    .on_conflict(ingredients::name)
                    .do_update()
                    .set(ingredients::name.eq(excluded(ingredients::name)))
                    .get_results(conn)
            )?;

            let inserted_recipe_ingredients: Vec<pg::RecipeIngredient> = {
                let new_recipe_ingredients: Vec<pg::NewRecipeIngredient> = inserted_ingredients
                    .iter()
                    .zip(recipe.ingredients.iter())
                    .map(|(pg_i, gql_i)| pg::NewRecipeIngredient {
                        ingredient_id: pg_i.id,
                        recipe_id: inserted_recipe.id,
                        qty: gql_i.qty.clone(),
                    })
                    .collect();

                as_field_result(
                    diesel::insert_into(recipe_ingredients::table)
                        .values(new_recipe_ingredients)
                        .on_conflict((recipe_ingredients::recipe_id, recipe_ingredients::ingredient_id))
                        .do_update()
                        .set(recipe_ingredients::qty.eq(excluded(recipe_ingredients::qty)))
                        .get_results(conn)
                )?
            };

            let zipped: Vec<(pg::RecipeIngredient, pg::Ingredient)> = inserted_recipe_ingredients
                .iter()
                .cloned()
                .zip(inserted_ingredients.iter().cloned())
                .collect();

            Ok(gql::Recipe::from_pg(
                &inserted_recipe,
                &zipped,
            ))
        })
    }

    pub fn delete_recipe(conn: &PgConnection, recipe_name: &str) -> FieldResult<()> {
        conn.transaction(|| {
            let found_recipe: pg::Recipe = as_field_result(
                recipes::table
                    .filter(recipes::name.eq(recipe_name))
                    .get_result::<pg::Recipe>(conn)
            )?;

            let found_ingredients: Vec<(pg::RecipeIngredient, pg::Ingredient)> = as_field_result(
                pg::RecipeIngredient::belonging_to(&found_recipe)
                    .inner_join(ingredients::table)
                    .get_results::<(pg::RecipeIngredient, pg::Ingredient)>(conn)
            )?;

            found_ingredients
                    .iter()
                    .map(|(ri, i)| Self::delete_ingredient_link(ri, i, conn))
                    .collect::<FieldResult<()>>()?;

            as_field_result(
                diesel::delete(&found_recipe)
                    .execute(conn)
                    .map(|_| ())
            )
        })
    }

    fn delete_ingredient_link(
        recipe_ingredient: &pg::RecipeIngredient,
        ingredient: &pg::Ingredient,
        conn: &PgConnection,
    ) -> FieldResult<()> {
        as_field_result(diesel::delete(recipe_ingredient).execute(conn))?;
        if Self::is_orphan_ingredient(ingredient, conn)? {
            as_field_result(diesel::delete(ingredient).execute(conn))?;
        }

        Ok(())
    }

    fn is_orphan_ingredient(ingredient: &pg::Ingredient, conn: &PgConnection) -> FieldResult<bool> {
        let links: Vec<pg::RecipeIngredient> = as_field_result(
            pg::RecipeIngredient::belonging_to(ingredient).get_results::<pg::RecipeIngredient>(conn)
        )?;

        Ok(links.is_empty())
    }
}