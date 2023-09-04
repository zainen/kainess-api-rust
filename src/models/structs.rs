use diesel::{prelude::Associations, AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(
  PartialEq, Serialize, Deserialize, Identifiable, Debug, Clone, Queryable, Selectable, AsChangeset,
)]
#[diesel(table_name = crate::models::schema::recipe)]
pub struct Recipe {
  #[diesel(sql_type = Integer)]
  pub id: i32,
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

#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::models::schema::recipe)]
pub struct NewRecipe {
  pub name: String,
  pub description: Option<String>,
  pub hidden: Option<bool>,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::models::schema::recipe_ingredient)]
pub struct NewIngredient {
  pub recipe_id: Option<i32>,
  pub name: String,
  pub quantity: Option<String>,
  pub measurement_type: Option<String>,
}
#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::models::schema::recipe_step)]
pub struct NewStep {
  pub recipe_id: Option<i32>,
  pub step_number: i32,
  pub step_directions: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateRecipe {
  pub recipe: NewRecipe,
  pub ingredients: Vec<NewIngredient>,
  pub steps: Vec<NewStep>,
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
pub struct UpdateSuccessIngredient {
  pub success: bool,
  pub ingredient: RecipeIngredient,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateSuccessStep {
  pub success: bool,
  pub step: RecipeStep,
}

// helper structs

#[derive(Serialize)]
pub struct Response {
  pub message: String,
}
