use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::fmt::Error;

use crate::models::schema::recipe::dsl::{id as id_of_recipe, recipe};
use crate::models::schema::recipe_ingredient::dsl::{
  recipe_id as ingredient_recipe_id, recipe_ingredient,
};
use crate::models::schema::recipe_step::dsl::{recipe_id as step_recipe_id, recipe_step};
use crate::models::{
  structs::{
    NewIngredient, NewRecipe, NewStep, Recipe, RecipeIngredient, RecipeStep, RecipeWithDetails,
  },
  types::GetAllRecipes,
};

type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
  pool: DBPool,
}

impl Database {
  pub fn new() -> Self {
    dotenv().ok();
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DBPool = r2d2::Pool::builder()
      .build(manager)
      .expect("failed to create pool");
    Database { pool }
  }

  pub fn get_recipes(&self) -> GetAllRecipes {
    recipe
      .load::<Recipe>(&mut self.pool.get().unwrap())
      .expect("Missing")
  }

  pub fn get_recipe_details(&self, target_id: i32) -> RecipeWithDetails {
    let found_recipe = recipe
      .filter(id_of_recipe.eq(target_id))
      .load::<Recipe>(&mut self.pool.get().unwrap())
      .unwrap()
      .pop();

    let found_ingredients = recipe_ingredient
      .filter(ingredient_recipe_id.eq(target_id))
      .load::<RecipeIngredient>(&mut self.pool.get().unwrap())
      .unwrap();

    let found_steps = recipe_step
      .filter(step_recipe_id.eq(target_id))
      .load::<RecipeStep>(&mut self.pool.get().unwrap())
      .unwrap();

    RecipeWithDetails {
      recipe: found_recipe,
      ingredients: found_ingredients,
      steps: found_steps,
    }
  }

  pub fn create_recipe(
    &self,
    new_recipe: NewRecipe,
    ingredients: Vec<NewIngredient>,
    steps: Vec<NewStep>,
  ) -> Result<(), Error> {
    let inserted_row: Result<Recipe, diesel::result::Error> = diesel::insert_into(recipe)
      .values(new_recipe)
      .get_result::<Recipe>(&mut self.pool.get().unwrap());

    let new_id = inserted_row.unwrap().id;
    for ingredient in ingredients {
      diesel::insert_into(recipe_ingredient)
        .values(NewIngredient {
          recipe_id: Some(new_id),
          ..ingredient
        })
        .execute(&mut self.pool.get().unwrap())
        .expect("failed to insert ingredient");
    }

    for step in steps {
      diesel::insert_into(recipe_step)
        .values(NewStep {
          recipe_id: Some(new_id),
          ..step
        })
        .execute(&mut self.pool.get().unwrap())
        .expect("failed to insert recipe step");
    }
    Ok(())
  }

  pub fn update_recipe(&self, target_recipe: Recipe) -> Option<Recipe> {
    let updated_recipe = diesel::update(recipe.find(target_recipe.id))
      .set(&target_recipe)
      .get_result::<Recipe>(&mut self.pool.get().unwrap())
      .expect(format!("Error updating recipe by id: {}", target_recipe.id).as_str());
    Some(updated_recipe)
  }

  pub fn update_ingredient(&self, target_ingredient: RecipeIngredient) -> Option<RecipeIngredient> {
    let updated_ingredient = diesel::update(recipe_ingredient.find(target_ingredient.id))
      .set(&target_ingredient)
      .get_result::<RecipeIngredient>(&mut self.pool.get().unwrap())
      .expect(format!("Error updating ingredient by id: {}", target_ingredient.id).as_str());
    Some(updated_ingredient)
  }

  pub fn update_step(&self, target_step: RecipeStep) -> Option<RecipeStep> {
    let updated_step = diesel::update(recipe_step.find(target_step.id))
      .set(&target_step)
      .get_result::<RecipeStep>(&mut self.pool.get().unwrap())
      .expect(format!("Error updating step by id: {}", target_step.id).as_str());
    Some(updated_step)
  }
}
