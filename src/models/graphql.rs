use crate::models::postgres;

#[derive(juniper::GraphQLObject)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<Ingredient>
}

impl Recipe {
    pub fn from_pg(
        pg_recipe: &postgres::Recipe,
        pg_ingredients: &Vec<postgres::Ingredient>
    ) -> Self {
        Self {
            name: pg_recipe.name.clone(),
            ingredients: pg_ingredients.iter().map(Ingredient::from_pg).collect()
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct Ingredient {
    pub name: String,
    pub qty: Option<String>
}

impl Ingredient {
    pub fn from_pg(pg_ingredient: &postgres::Ingredient) -> Self {
        Self {
            name: pg_ingredient.name.clone(),
            qty: pg_ingredient.qty.clone()
        }
    }
}