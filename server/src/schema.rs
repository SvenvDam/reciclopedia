table! {
    ingredients (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    recipe_ingredients (recipe_id, ingredient_id) {
        recipe_id -> Int4,
        ingredient_id -> Int4,
        qty -> Nullable<Varchar>,
    }
}

table! {
    recipes (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    users (username) {
        username -> Varchar,
        salt -> Varchar,
        hashpwd -> Varchar,
        token -> Nullable<Varchar>,
    }
}

joinable!(recipe_ingredients -> ingredients (ingredient_id));
joinable!(recipe_ingredients -> recipes (recipe_id));

allow_tables_to_appear_in_same_query!(
    ingredients,
    recipe_ingredients,
    recipes,
    users,
);
