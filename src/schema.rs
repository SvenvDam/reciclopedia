table! {
    ingredients (id) {
        id -> Int4,
        recipe_id -> Int4,
        name -> Varchar,
        qty -> Nullable<Varchar>,
    }
}

table! {
    recipes (id) {
        id -> Int4,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    ingredients,
    recipes,
);
