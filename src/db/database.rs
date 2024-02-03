use diesel::r2d2::{self, ConnectionManager};
#[allow(deprecated)]
use diesel::{dsl::any, prelude::*, ExpressionMethods};
use dotenv::dotenv;
use std::{collections::HashSet, fmt::Error, sync::Arc};

use bcrypt::{hash, verify};

use crate::models::{
  schema::herbs::{
    dsl::{function, herbs, id as herb_db_id, tcm_name, tcm_name_en},
    indication, meridians, properties,
  },
  structs::{HerbCollectionJist, SearchKeywords, Temp},
  types::HerbVecJist,
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

mod sql_helper {
  macro_rules! str_partial_eq {
    ($str:expr ) => {
      format!("%{}%", $str)
    };
  }
  pub(crate) use str_partial_eq;
}

type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
  pool: DBPool,
}

// RECIPE DB FN
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
}

// USER DATABASE FN
impl Database {
  pub fn create_user(
    &self,
    user: UserValidationParams,
  ) -> Result<UserJwtInfo, diesel::result::Error> {
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
}

const PAGE_LIMIT: i32 = 30;
// TCM DATABASE FN
impl Database {
  // TODO rename
  pub fn get_herb_count(&self) -> Vec<i32> {
    let herb_ids: Vec<i32> = herbs
      .select(herb_db_id)
      .filter(function.is_not_null())
      .load::<i32>(&mut self.pool.get().unwrap())
      .unwrap();

    let page_count = ((herb_ids.len() / PAGE_LIMIT as usize) as f32).floor() as i32;

    (0..=page_count)
      .into_iter()
      .map(|x| herb_ids[(x * PAGE_LIMIT) as usize])
      .collect()
  }

  // LIMIT PAGE_LIMIT value herbs per call
  pub fn get_herbs_limit(
    &self,
    start_from_herb_id: i32,
  ) -> Result<HerbVecJist, diesel::result::Error> {
    Arc::new(
      herbs
        .select(HerbCollectionJist::as_select())
        .filter(function.is_not_null())
        .filter(herb_db_id.ge(start_from_herb_id))
        .limit(PAGE_LIMIT as i64),
    )
    .load::<HerbCollectionJist>(&mut self.pool.get().unwrap())
  }

#[deprecated]
  pub fn _deprecated_search_herbs_keywords(
    &self,
    search_params: SearchKeywords,
  ) -> Result<HerbVecJist, diesel::result::Error> {

    // dont care if jumk meridians are added since check is for any that possibly match not equal all
    let SearchKeywords {
      herb_name,
      herb_name_cn,
      herb_function,
      herb_meridians,
      herb_indication,
      herb_properties,
    } = search_params;

    let herb_name_fmt = herb_name
      .iter()
      .map(|en_name| sql_helper::str_partial_eq!(en_name))
      .collect::<Vec<String>>();

    let herb_name_cn_fmt = herb_name_cn
      .iter()
      .map(|cn_name| sql_helper::str_partial_eq!(cn_name))
      .collect::<Vec<String>>();

    let herb_function_fmt = herb_function
      .iter()
      .map(|func| sql_helper::str_partial_eq!(func))
      .collect::<Vec<String>>();

    let herb_meridians_fmt = herb_meridians
      .iter()
      .map(|merid| sql_helper::str_partial_eq!(merid))
      .collect::<Vec<String>>();

    let herb_indication_fmt = herb_indication
      .iter()
      .map(|ind| sql_helper::str_partial_eq!(ind))
      .collect::<Vec<String>>();

    let herb_properties_fmt = herb_properties
      .iter()
      .map(|property| sql_helper::str_partial_eq!(property))
      .collect::<Vec<String>>();

    // TODO KEEP AN EYE ON DIESEL DOCS FOR REPLACEMENTS FOR THE DEPRECATED any CURRENTLY THE ONLY SOLUTION TO THIS USE CASE
    #[allow(deprecated)]
    herbs
      .select(HerbCollectionJist::as_select())
      .filter(tcm_name_en.ilike(any(herb_name_fmt)))
      .or_filter(tcm_name.ilike(any(herb_name_cn_fmt)))
      .filter(function.ilike(any(herb_function_fmt)).is_not_null())
      .filter(meridians.ilike(any(herb_meridians_fmt)))
      .filter(indication.ilike(any(herb_indication_fmt)))
      .filter(properties.ilike(any(herb_properties_fmt)))
      .load::<HerbCollectionJist>(&mut self.pool.get().unwrap())
  }

  pub fn search_herbs_keywords_count(
    &self,
    herb_meridians: Vec<String>,
  ) -> Vec<i32> {

    // ONLY CHECK MERIDIANS since name is less clear

    let herb_meridians_fmt = herb_meridians
      .iter()
      .map(|merid| sql_helper::str_partial_eq!(merid))
      .collect::<Vec<String>>();

    // TODO KEEP AN EYE ON DIESEL DOCS FOR REPLACEMENTS FOR THE DEPRECATED any CURRENTLY THE ONLY SOLUTION TO THIS USE CASE
    let mut query = herbs
      .select(herb_db_id)
      .filter(function.is_not_null())
      .filter(meridians.is_not_null())
      .into_boxed();
    for m in herb_meridians_fmt.iter() {
      query = query
        .filter(meridians.ilike(m));
    }
    let herb_ids = query.load::<i32>(&mut self.pool.get().unwrap()).unwrap();

    let page_count = ((herb_ids.len() / PAGE_LIMIT as usize) as f32).floor() as i32;

    (0..=page_count)
      .into_iter()
      .map(|x| herb_ids[(x * PAGE_LIMIT) as usize])
      .collect()
  }

  pub fn search_herbs_keywords(
    &self,
    page_index: i32,
    herb_meridians: Vec<String>,
  ) -> Result<HerbVecJist, diesel::result::Error> {

    // ONLY CHECK MERIDIANS since name is less clear

    let herb_meridians_fmt = herb_meridians
      .iter()
      .map(|merid| sql_helper::str_partial_eq!(merid))
      .collect::<Vec<String>>();

    // TODO KEEP AN EYE ON DIESEL DOCS FOR REPLACEMENTS FOR THE DEPRECATED any CURRENTLY THE ONLY SOLUTION TO THIS USE CASE
    let mut query = herbs
      .select(HerbCollectionJist::as_select())
      .filter(function.is_not_null())
      .filter(meridians.is_not_null())
      .filter(herb_db_id.ge(page_index))
      .into_boxed();
    for m in herb_meridians_fmt.iter() {
      query = query
        .filter(meridians.ilike(m));
    }
    query
      .limit(PAGE_LIMIT as i64)
      .load::<HerbCollectionJist>(&mut self.pool.get().unwrap())
  }

  pub fn get_herb_information(&self, herb_id: i32) -> Result<Vec<Herb>, diesel::result::Error> {
    herbs
      .filter(herb_db_id.eq(herb_id))
      .load::<Herb>(&mut self.pool.get().unwrap())
  }

  pub fn unique_meridians(&self) -> Result<Vec<String>, diesel::result::Error> {
    let query: Result<Vec<Temp>, diesel::result::Error> = herbs
      .select(Temp::as_select())
      .distinct_on(meridians)
      .filter(meridians.is_not_null())
      .load::<Temp>(&mut self.pool.get().unwrap());

    match query {
      Ok(query) => {
        let mut set: HashSet<String> = HashSet::new();
        for temp_item in query.iter() {
          let temp_dirty_meridians = &temp_item.meridians;
          if let Some(dirty_merids) = temp_dirty_meridians {
            let clean_group = dirty_merids.trim().split(";");
            for clean_merid in clean_group {
              set.insert(
                clean_merid
                  .to_string()
                  .trim()
                  .to_ascii_lowercase()
                  .to_string(),
              );
            }
          }
        }
        // needed for constant order
        let mut unique_meridians = Vec::from_iter(set);
        unique_meridians.sort();
        Ok(unique_meridians)
      }
      Err(e) => Err(e),
    }
  }
}
