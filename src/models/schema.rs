// @generated automatically by Diesel CLI.

diesel::table! {
    recipe (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        hidden -> Nullable<Bool>,
    }
}

diesel::table! {
    recipe_ingredient (id) {
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
    recipe_step (id) {
        id -> Int4,
        recipe_id -> Int4,
        step_number -> Int4,
        step_directions -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::joinable!(recipe -> users (user_id));
diesel::joinable!(recipe_ingredient -> recipe (recipe_id));
diesel::joinable!(recipe_step -> recipe (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(recipe, recipe_ingredient, recipe_step, users,);
