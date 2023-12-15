// @generated automatically by Diesel CLI.

diesel::table! {
    herbs (id) {
        id -> Int4,
        #[max_length = 16]
        tcmbank_id -> Varchar,
        #[max_length = 255]
        level1_name_en -> Nullable<Varchar>,
        #[max_length = 255]
        level2_name -> Nullable<Varchar>,
        #[max_length = 255]
        tcm_name -> Nullable<Varchar>,
        tcm_name_en -> Nullable<Text>,
        #[max_length = 255]
        herb_pinyin_name -> Nullable<Varchar>,
        #[max_length = 255]
        herb_latin_name -> Nullable<Varchar>,
        properties -> Nullable<Text>,
        meridians -> Nullable<Text>,
        usepart -> Nullable<Text>,
        function -> Nullable<Text>,
        indication -> Nullable<Text>,
        #[max_length = 255]
        toxicity -> Nullable<Varchar>,
        #[max_length = 255]
        clinical_manifestations -> Nullable<Varchar>,
        #[max_length = 255]
        therapeutic_en_class -> Nullable<Varchar>,
        #[max_length = 255]
        therapeutic_cn_class -> Nullable<Varchar>,
        #[max_length = 50]
        tcmid_id -> Nullable<Varchar>,
        tcm_id_id -> Nullable<Int4>,
        symmap_id -> Nullable<Int4>,
        tcmsp_id -> Nullable<Int4>,
        #[max_length = 66]
        herb_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    recipe (id) {
        id -> Int4,
        creator_id -> Int4,
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
        email -> Varchar,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        is_admin -> Bool,
    }
}

diesel::joinable!(recipe -> users (creator_id));
diesel::joinable!(recipe_ingredient -> recipe (recipe_id));
diesel::joinable!(recipe_step -> recipe (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(herbs, recipe, recipe_ingredient, recipe_step, users,);
