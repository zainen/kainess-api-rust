use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::{collections::HashSet, fmt::Error, sync::Arc};

use bcrypt::{hash, verify};

use crate::models::{
  schema::herbs::dsl::{function, herbs, id as herb_db_id},
  structs::{QueryHerbs, SearchKeywords},
  types::HerbVec,
};
use crate::models::{
  schema::recipe_step::dsl::{
    id as step_id, recipe_id as step_recipe_id, recipe_step, step_number,
  },
  structs::{Herb, Response},
};
use crate::models::{
  schema::users::dsl::{email as email_column, users},
  structs::User,
};
use crate::models::{
  structs::{
    NewRecipe, NewRecipeIngredient, NewRecipeStep, Recipe, RecipeIngredient, RecipeStep,
    RecipeWithDetails,
  },
  types::GetAllRecipes,
};
use crate::{
  api::auth::structs::LoginParams,
  models::{
    schema::recipe::dsl::{id as id_of_recipe, recipe},
    structs::GeneralDbQuerySuccess,
  },
};
use crate::{
  api::auth::structs::UserJwtInfo,
  models::{
    schema::recipe_ingredient::dsl::{recipe_id as ingredient_recipe_id, recipe_ingredient},
    structs::UserValidationParams,
  },
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

  pub fn get_recipe(&self, target_id: i32) -> Option<Recipe> {
    recipe
      .filter(id_of_recipe.eq(target_id))
      .load::<Recipe>(&mut self.pool.get().unwrap())
      .unwrap()
      .pop()
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
    ingredients: Vec<NewRecipeIngredient>,
    steps: Vec<NewRecipeStep>,
  ) -> Result<(), Error> {
    let inserted_row: Result<Recipe, diesel::result::Error> = diesel::insert_into(recipe)
      .values(new_recipe)
      .get_result::<Recipe>(&mut self.pool.get().unwrap());

    let new_id = inserted_row.unwrap().id;
    for ingredient in ingredients {
      diesel::insert_into(recipe_ingredient)
        .values(NewRecipeIngredient {
          recipe_id: Some(new_id),
          ..ingredient
        })
        .execute(&mut self.pool.get().unwrap())
        .expect("failed to insert ingredient");
    }

    for step in steps {
      diesel::insert_into(recipe_step)
        .values(NewRecipeStep {
          recipe_id: Some(new_id),
          ..step
        })
        .execute(&mut self.pool.get().unwrap())
        .expect("failed to insert recipe step");
    }
    // TODO update return
    Ok(())
  }

  pub fn add_recipe_step(
    &self,
    new_step: NewRecipeStep,
    recipe_id: i32,
    new_ingredient_position: i32,
  ) -> Vec<RecipeStep> {
    let mut all_steps: Vec<RecipeStep> = recipe_step
      .filter(step_recipe_id.eq(recipe_id))
      .order(step_number.asc())
      .load::<RecipeStep>(&mut self.pool.get().unwrap())
      .unwrap();
    let inserted_step = diesel::insert_into(recipe_step)
      .values(NewRecipeStep {
        recipe_id: Some(recipe_id),
        ..new_step
      })
      .get_result::<RecipeStep>(&mut self.pool.get().unwrap())
      .expect("failed to insert recipe step");

    // place new stop in right place
    all_steps.insert(new_ingredient_position as usize, inserted_step);

    for (i, step) in all_steps.iter().enumerate() {
      diesel::update(recipe_step)
        .filter(step_id.eq(step.id))
        .set(step_number.eq(i as i32 + 1_i32))
        .execute(&mut self.pool.get().unwrap())
        .expect(&format!("failed to update step {:?}", step));
    }

    recipe_step
      .filter(step_recipe_id.eq(recipe_id))
      .order(step_number.asc())
      .load::<RecipeStep>(&mut self.pool.get().unwrap())
      .unwrap()
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

  pub fn delete_recipe(&self, target_recipe: Recipe) -> GeneralDbQuerySuccess {
    match diesel::delete(recipe.find(target_recipe.id)).execute(&mut self.pool.get().unwrap()) {
      Ok(_) => GeneralDbQuerySuccess { success: true },
      Err(_) => GeneralDbQuerySuccess { success: false },
    }
  }

  pub fn delete_recipe_ingredient(
    &self,
    target_ingredient: RecipeIngredient,
  ) -> GeneralDbQuerySuccess {
    diesel::delete(recipe_ingredient.find(target_ingredient.id))
      .execute(&mut self.pool.get().unwrap())
      .expect("Failed to delete the recipe ingredient");
    GeneralDbQuerySuccess { success: true }
  }
  pub fn delete_recipe_step(&self, target_step: RecipeStep) -> GeneralDbQuerySuccess {
    diesel::delete(recipe_step.find(target_step.id))
      .execute(&mut self.pool.get().unwrap())
      .expect("failed to delete the recipe step");
    let found_steps: Vec<RecipeStep> = recipe_step
      .filter(step_recipe_id.eq(target_step.recipe_id))
      .order(step_number.asc())
      .load::<RecipeStep>(&mut self.pool.get().unwrap())
      .unwrap();
    for (i, step) in found_steps.iter().enumerate() {
      diesel::update(recipe_step)
        .filter(step_id.eq(step.id))
        .set(step_number.eq(i as i32))
        .execute(&mut self.pool.get().unwrap())
        .expect("failed to update step number");
    }
    GeneralDbQuerySuccess { success: true }
  }

  pub fn create_user(
    &self,
    user: UserValidationParams,
  ) -> Result<UserJwtInfo, diesel::result::Error> {
    println!("starting db insert");
    dotenv().ok();
    let hash = hash(user.password, 14).expect("FAILED TO HASH");
    let UserValidationParams {
      email,
      first_name,
      last_name,
      is_admin,
      ..
    } = user;

    let new_user = UserValidationParams {
      email,
      first_name,
      last_name,
      password: hash,
      is_admin,
    };
    match diesel::insert_into(users)
      .values(new_user)
      .get_result::<User>(&mut self.pool.get().unwrap())
    {
      Ok(new_user) => {
        let User {
          id,
          email,
          first_name,
          last_name,
          is_admin,
          .. // unused rest of User struct
        } = new_user;
        Ok(UserJwtInfo {
          id,
          email,
          first_name,
          last_name,
          is_admin,
        })
      }
      Err(e) => Err(e),
    }
  }

  pub fn check_user(&self, creds: LoginParams) -> Result<UserJwtInfo, Response> {
    let found_user: Option<User> = users
      .filter(email_column.eq(&creds.email))
      .load::<User>(&mut self.pool.get().unwrap())
      .unwrap()
      .pop();
    match found_user {
      Some(user) => {
        let extend_user = user.clone();
        if verify(creds.password, &user.password).unwrap() {
          let payload = UserJwtInfo {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            is_admin: user.is_admin,
          };
          if creds.email != extend_user.email {
            return Err(Response {
              message: "Email Error".to_string(),
            });
          }
          Ok(payload)
        } else {
          Err(Response {
            message: "Failed to verify user".to_string(),
          })
        }
      }
      None => Err(Response {
        message: "Failed to verify user".to_string(),
      }),
    }
  }

  // TCM DB FUNCTIONS

  // LIMIT 10 herbs per call
  pub fn get_herbs(&self, start_from_herb_id: i32) -> Result<HerbVec, diesel::result::Error> {
    let filtered_herbs = Arc::new(
      herbs
        .select(Herb::as_select())
        .filter(function.is_not_null())
        .filter(herb_db_id.gt(start_from_herb_id))
        .order_by(herb_db_id.asc())
        .limit(100),
    )
    .load::<Herb>(&mut self.pool.get().unwrap())
    .unwrap();

    Ok(filtered_herbs)
  }

  // TODO update error
  pub fn search_herbs(&self, search_params: SearchKeywords) -> Result<HerbVec, ()> {
    let iterator = search_params.keywords.iter();

    // get all unique keys with function related to key words
    let mut id_set: HashSet<i32> = HashSet::new();
    for iter in iterator {
      let fmt = format!("%{}%", iter);
      let filtered_herbs: Vec<QueryHerbs> = Arc::new(
        herbs
          .select(QueryHerbs::as_select())
          .filter(function.like(&fmt))
          .order_by(herb_db_id.asc()),
      )
      .limit(100)
      .load::<QueryHerbs>(&mut self.pool.get().unwrap())
      .unwrap();

      for QueryHerbs { id } in filtered_herbs.iter() {
        id_set.insert(*id);
      }
    }

    let filtered_herbs: HerbVec = herbs
      .select(Herb::as_select())
      .filter(function.is_not_null())
      .filter(herb_db_id.eq_any(id_set.clone()))
      .order_by(herb_db_id.asc())
      .limit(100)
      .load::<Herb>(&mut self.pool.get().unwrap())
      .unwrap();

    Ok(filtered_herbs)
  }

  // TODO update error
  pub fn get_herb_information(&self, herb_id: i32) -> Result<Vec<Herb>, diesel::result::Error> {
    herbs.filter(herb_db_id.eq(herb_id)).load::<Herb>(&mut self.pool.get().unwrap())
  }
}
