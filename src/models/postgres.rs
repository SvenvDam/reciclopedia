use crate::schema::*;

#[derive(Queryable, Identifiable, Clone, Debug)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Identifiable, Clone, Debug, PartialEq)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Identifiable, Associations, Clone)]
#[belongs_to(Recipe)]
#[belongs_to(Ingredient)]
pub struct RecipeIngredient {
    pub id: i32,
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub qty: Option<String>,
}
