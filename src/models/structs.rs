use diesel::{prelude::Associations, AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use serde_with::{StringWithSeparator, formats::CommaSeparator};

use super::types::HerbVecJist;

#[derive(
  PartialEq, Serialize, Deserialize, Identifiable, Debug, Clone, Queryable, Selectable, AsChangeset,
)]
#[diesel(table_name = crate::models::schema::users)]
pub struct User {
  pub id: i32,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub password: String,
  pub is_admin: bool,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::models::schema::users)]
pub struct UserValidationParams {
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub password: String,
  pub is_admin: Option<bool>,
}

#[derive(
  PartialEq, Serialize, Deserialize, Identifiable, Debug, Clone, Queryable, Selectable, AsChangeset,
)]
#[diesel(table_name = crate::models::schema::recipe)]
pub struct Recipe {
  #[diesel(sql_type = Integer)]
  pub id: i32,
  pub creator_id: i32,
  pub name: String,
  pub description: Option<String>,
  pub hidden: Option<bool>,
}

#[derive(
  PartialEq,
  Serialize,
  Deserialize,
  Associations,
  Identifiable,
  Debug,
  Clone,
  Queryable,
  Selectable,
  AsChangeset,
)]
#[diesel(belongs_to(Recipe, foreign_key = recipe_id))]
#[diesel(table_name = crate::models::schema::recipe_ingredient)]
pub struct RecipeIngredient {
  pub id: i32,
  pub recipe_id: i32,
  pub name: String,
  pub quantity: Option<String>,
  pub measurement_type: Option<String>,
}

#[derive(
  PartialEq,
  Serialize,
  Deserialize,
  Associations,
  Identifiable,
  Debug,
  Clone,
  Queryable,
  Selectable,
  AsChangeset,
)]
#[diesel(belongs_to(Recipe, foreign_key = recipe_id))]
#[diesel(table_name = crate::models::schema::recipe_step)]
pub struct RecipeStep {
  pub id: i32,
  pub recipe_id: i32,
  pub step_number: i32,
  pub step_directions: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::models::schema::recipe)]
pub struct NewRecipe {
  pub name: String,
  pub creator_id: i32,
  pub description: Option<String>,
  pub hidden: Option<bool>,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::models::schema::recipe_ingredient)]
pub struct NewRecipeIngredient {
  pub recipe_id: Option<i32>,
  pub name: String,
  pub quantity: Option<String>,
  pub measurement_type: Option<String>,
}
#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::models::schema::recipe_step)]
pub struct NewRecipeStep {
  pub recipe_id: Option<i32>,
  pub step_number: i32,
  pub step_directions: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateRecipe {
  pub recipe: NewRecipe,
  pub ingredients: Vec<NewRecipeIngredient>,
  pub steps: Vec<NewRecipeStep>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecipeWithDetails {
  pub recipe: Option<Recipe>,
  pub ingredients: Vec<RecipeIngredient>,
  pub steps: Vec<RecipeStep>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateRecipeWithDetails {
  pub recipe: Option<Recipe>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateSuccessRecipe {
  pub success: bool,
  pub recipe: Recipe,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateSuccessRecipeIngredient {
  pub success: bool,
  pub ingredient: RecipeIngredient,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateSuccessRecipeStep {
  pub success: bool,
  pub recipe_step: RecipeStep,
}

// TCM DB
#[derive(Queryable, Serialize, Deserialize, Debug, Clone, Selectable, Identifiable)]
#[diesel(table_name = crate::models::schema::herbs)]
pub struct Herb {
  pub id: i32,
  pub tcmbank_id: String,
  pub level1_name_en: Option<String>,
  pub level2_name: Option<String>,
  pub tcm_name: Option<String>,
  pub tcm_name_en: Option<String>,
  pub herb_pinyin_name: Option<String>,
  pub herb_latin_name: Option<String>,
  pub properties: Option<String>,
  pub meridians: Option<String>,
  pub usepart: Option<String>,
  pub function: Option<String>,
  pub indication: Option<String>,
  pub toxicity: Option<String>,
  pub clinical_manifestations: Option<String>,
  pub therapeutic_en_class: Option<String>,
  pub therapeutic_cn_class: Option<String>,
  pub tcmid_id: Option<String>,
  pub tcm_id_id: Option<i32>,
  pub symmap_id: Option<i32>,
  pub tcmsp_id: Option<i32>,
  pub herb_id: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, Selectable, Identifiable)]
#[diesel(table_name = crate::models::schema::herbs)]
pub struct Temp {
  pub id: i32,
  pub meridians: Option<String>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone, Identifiable)]
#[diesel(table_name = crate::models::schema::herbs)]
pub struct HerbCollectionJist {
  pub id: i32,
  pub tcm_name: Option<String>,
  pub tcm_name_en: Option<String>,
  pub herb_pinyin_name: Option<String>,
  pub herb_latin_name: Option<String>,
  pub properties: Option<String>,
  pub meridians: Option<String>,
  pub therapeutic_en_class: Option<String>,
  pub therapeutic_cn_class: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeywordFoundHerbs {
  pub herbs: HerbVecJist,
  pub pages: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JWT {
  token: String,
}

// helper structs

#[derive(Serialize)]
pub struct Response {
  pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneralDbQuerySuccess {
  pub success: bool,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, Selectable, Identifiable)]
#[diesel(table_name = crate::models::schema::herbs)]
pub struct QueryHerbs {
  pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchKeywords {
  pub herb_name: Vec<String>,
  pub herb_name_cn: Vec<String>,
  pub herb_function: Vec<String>,
  pub herb_meridians: Vec<String>,
  pub herb_indication: Vec<String>,
  pub herb_properties: Vec<String>,
}

#[serde_with::serde_as]
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchMeridians {
  #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
  pub meridians: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetHerbs {
  pub herbs: HerbVecJist,
  pub pages: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMeridians {
  pub meridians: Vec<String>,
}
