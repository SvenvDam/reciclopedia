use crate::models::graphql;
use crate::schema::*;

#[derive(Queryable, Identifiable, Insertable, Clone, Debug, PartialEq)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Clone, Debug)]
#[table_name = "recipes"]
pub struct NewRecipe {
    pub name: String
}

impl NewRecipe {
    pub fn from_graphql(rcp: &graphql::NewRecipe) -> Self {
        Self {
            name: rcp.name.clone()
        }
    }
}

#[derive(Queryable, Identifiable, Insertable, Clone, Debug, PartialEq)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Clone, Debug)]
#[table_name = "ingredients"]
pub struct NewIngredient {
    pub name: String
}

impl NewIngredient {
    pub fn from_graphql(ing: &graphql::NewIngredient) -> Self {
        Self {
            name: ing.name.clone()
        }
    }

    pub fn from_graphql_many(ings: &[graphql::NewIngredient]) -> Vec<Self> {
        ings.iter().map(Self::from_graphql).collect()
    }
}

#[derive(Queryable, Identifiable, Insertable, Associations, Clone, Debug, PartialEq)]
#[primary_key(recipe_id, ingredient_id)]
#[belongs_to(Recipe)]
#[belongs_to(Ingredient)]
pub struct RecipeIngredient {
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub qty: Option<String>,
}

#[derive(Insertable, Clone, Debug)]
#[table_name = "recipe_ingredients"]
pub struct NewRecipeIngredient {
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub qty: Option<String>,
}

#[derive(Queryable, Identifiable, Debug, Insertable, AsChangeset, PartialEq)]
#[primary_key(username)]
pub struct User {
    pub username: String,
    pub salt: String,
    pub hashpwd: String,
    pub token: Option<String>,
}
