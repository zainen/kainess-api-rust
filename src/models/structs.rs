use serde::{Deserialize, Serialize};
use diesel::{Queryable, Selectable, Insertable, AsChangeset};

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::models::schema::recipes)]
pub struct Recipes {
  #[diesel(sql_type = Integer)]
  pub id: i32,
  pub name: Option<String>,
  pub description: Option<String>
}



#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::models::schema::recipe_ingredients)]
pub struct RecipeIngredients {
  pub id: i32,
  pub recipe_id: i32,
  pub name: String,
  pub quantity: Option<String>,
  pub measurement_type: Option<String>
}


#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::models::schema::recipe_steps)]
pub struct RecipeSteps{
  pub id: i32,
  pub recipe_id: i32,
  pub step_number: i32,
  pub step_directions: String
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = crate::models::schema::recipes)]
pub struct NewRecipe {
  pub name: String,
  pub description: Option<String>
}

#[derive(Insertable, )]
#[diesel(table_name = crate::models::schema::recipe_ingredients)]
pub struct NewIngredient {
  pub recipe_id: i32,
  pub name: String,
  pub quantity: String,
  pub measurement_type: String
}
#[derive(Insertable, )]
#[diesel(table_name = crate::models::schema::recipe_steps)]
pub struct NewStep {
  pub recipe_id: i32,
  pub step_number: i32,
  pub step_directions: String
}