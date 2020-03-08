use crate::schema::ingredients;
use crate::schema::recipes;

#[derive(Queryable, Identifiable, Clone)]
pub struct Recipe {
    pub id: i32,
    pub name: String
}

#[derive(Queryable, Identifiable, Associations, Clone)]
#[belongs_to(Recipe)]
pub struct Ingredient {
    pub id: i32,
    pub recipe_id: i32, // automatically recognized as foreign key by diesel
    pub name: String,
    pub qty: Option<String>
}

