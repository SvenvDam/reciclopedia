use crate::models::postgres;

#[derive(juniper::GraphQLObject)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<Ingredient>,
}

impl Recipe {
    pub fn from_pg(
        recipe: &postgres::Recipe,
        ingredients: &Vec<(postgres::RecipeIngredient, postgres::Ingredient)>,
    ) -> Self {
        Self {
            name: recipe.name.clone(),
            ingredients: ingredients
                .iter()
                .map(|(ri, i)| Ingredient::from_pg(&ri, &i))
                .collect(),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct Ingredient {
    pub name: String,
    pub qty: Option<String>,
}

impl Ingredient {
    pub fn from_pg(
        recipe_ingredient: &postgres::RecipeIngredient,
        ingredient: &postgres::Ingredient,
    ) -> Self {
        Self {
            name: ingredient.name.clone(),
            qty: recipe_ingredient.qty.clone(),
        }
    }
}