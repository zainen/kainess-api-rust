use actix_web::web;

use crate::{db::database::Database, models::structs::{Recipe, RecipeIngredient}};

pub async fn db_update_recipe (db: web::Data<Database>, recipe: Recipe) -> Option<Recipe> {
  db.update_recipe(recipe)
  
}

pub async fn update_ingredient(db: web::Data<Database>, ingredient: RecipeIngredient) -> Option<RecipeIngredient> {
  db.update_ingredient(ingredient)
}