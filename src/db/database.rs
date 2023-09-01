use std::fmt::Error;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::{structs::{Recipes, RecipeIngredients, RecipeSteps, NewRecipe, NewIngredient, NewStep}, schema::recipe_ingredients};
use crate::models::schema::recipes::dsl::*;
use crate::models::schema::recipe_ingredients::dsl::*;
use crate::models::schema::recipe_steps::dsl::*;

type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
  pool: DBPool
}

impl Database {
  pub fn new() -> Self {
    dotenv().ok();
    let database_url: String = std::env::var("DATABASED_URL").expect("DATABASE_URL not found");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DBPool = r2d2::Pool::builder()
      .build(manager)
      .expect("failed to create pool");
    Database { pool }
  }

  pub fn get_recipes(&self) -> Vec<Recipes> {
    
    recipes
      .load::<Recipes>(&mut self.pool.get().unwrap())
      .expect("error loading recipes")
  }

  pub fn create_recipe(&self, recipe: NewRecipe, ingredients: Vec<NewIngredient>, steps: Vec<NewStep>) -> Result<(), Error> {
    let inserted_row: Result<Recipes, diesel::result::Error> = diesel::insert_into(recipes)
      .values(recipe)
      .get_result::<Recipes>(&mut self.pool.get().unwrap());

    dbg!(&inserted_row);
    let new_id = inserted_row.unwrap().id;
    for ingredient in ingredients {
      diesel::insert_into(recipe_ingredients)
        .values(NewIngredient {
          recipe_id: new_id,
          ..ingredient
        }).execute(&mut self.pool.get().unwrap())
        .expect("failed to insert ingredient");
    }

    for step in steps {
      diesel::insert_into(recipe_steps)
        .values(NewStep {
          recipe_id: new_id,
          ..step
        }).execute(&mut self.pool.get().unwrap())
        .expect("failed to insert recipe step");
    }
    Ok(())
  }

  pub fn update_recipe(&self, recipe: Recipes) -> Option<Recipes> {
    let updated_recipe = diesel::update(recipes.find(recipe.id))
      .set(&recipe)
      .get_result::<Recipes>(&mut self.pool.get().unwrap())
      .expect(format!("Error updating recipe by id: {}", recipe.id).as_str());
    Some(updated_recipe)
  }

  pub fn update_ingredient(&self, ingredient: RecipeIngredients) -> Option<RecipeIngredients> {
    let updated_ingredient = diesel::update(recipe_ingredients.find(ingredient.id))
      .set(&ingredient)
      .get_result::<RecipeIngredients>(&mut self.pool.get().unwrap())
      .expect(format!("Error updating ingredient by id: {}", ingredient.id).as_str());
    Some(updated_ingredient)
  }

  pub fn update_step(&self, step: RecipeSteps) -> Option<RecipeSteps> {
    let updated_step = diesel::update(recipe_steps.find(step.id))
      .set(&step)
      .get_result::<RecipeSteps>(&mut self.pool.get().unwrap())
      .expect(format!("Error updating step by id: {}", step.id).as_str());
    Some(updated_step)
  }
}