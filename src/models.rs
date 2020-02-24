#[derive(Queryable)]
pub struct Ingredient {
    pub id: i32,
    pub recipe_id: i32,
    pub name: String,
    pub qty: Option<String>
}

#[juniper::object()]
impl Ingredient {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn qty(&self) -> Option<&str> {
        self.qty.as_ref().map(|s| s.as_str())
    }
}

#[derive(Queryable)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub ingredients: Vec<Ingredient>
}

#[juniper::object()]
impl Recipe {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn ingredients(&self) -> Vec<Ingredient> {
        vec![]
    }
}