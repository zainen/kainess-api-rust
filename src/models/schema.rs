// @generated automatically by Diesel CLI.

diesel::table! {
    recipe_ingredients (id) {
        id -> Int4,
        recipe_id -> Int4,
        #[max_length = 20]
        name -> Varchar,
        #[max_length = 10]
        quantity -> Nullable<Varchar>,
        #[max_length = 10]
        measurement_type -> Nullable<Varchar>,
    }
}

diesel::table! {
    recipe_steps (id) {
        id -> Int4,
        recipe_id -> Int4,
        step_number -> Int4,
        step_directions -> Text,
    }
}

diesel::table! {
    recipes (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        description -> Nullable<Text>,
    }
}

diesel::joinable!(recipe_ingredients -> recipes (recipe_id));
diesel::joinable!(recipe_steps -> recipes (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(
    recipe_ingredients,
    recipe_steps,
    recipes,
);
