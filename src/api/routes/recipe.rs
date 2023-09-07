use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use crate::models::structs::{GeneralDbQuerySuccess, UpdateSuccessRecipeStep};
use crate::models::{
  schema::recipe_ingredient,
  structs::{
    CreateRecipe, RecipeWithDetails, Response, UpdateSuccessRecipe, UpdateSuccessRecipeIngredient,
  },
};
use crate::{
  db::database::Database,
  models::structs::{
    NewRecipe, NewRecipeIngredient, NewRecipeStep, Recipe, RecipeIngredient, RecipeStep,
  },
};

#[get("/")]
pub async fn get_recipes(db: web::Data<Database>) -> impl Responder {
  let recipes: Vec<Recipe> = db.get_recipes();
  HttpResponse::Ok().json(recipes)
}

#[post("/")]
pub async fn create_recipe(
  db: web::Data<Database>,
  recipe_information: web::Json<CreateRecipe>,
) -> impl Responder {
  let inner_info: CreateRecipe = recipe_information.into_inner();
  let recipe_to_create: NewRecipe = inner_info.recipe;
  let ingredients_to_insert: Vec<NewRecipeIngredient> = inner_info.ingredients;
  let steps_to_insert: Vec<NewRecipeStep> = inner_info.steps;

  let insert_result = db.create_recipe(recipe_to_create, ingredients_to_insert, steps_to_insert);

  match insert_result {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::NotAcceptable(),
  }
}

#[get("/{id}")]
pub async fn get_recipe_details(db: web::Data<Database>, id: web::Path<i32>) -> impl Responder {
  let recipe_with_details: RecipeWithDetails = db.get_recipe_details(*id);
  match &recipe_with_details.recipe {
    Some(_) => HttpResponse::Ok().json(recipe_with_details),
    None => {
      let response = Response {
        message: "Recipe Not Found".to_string(),
      };
      HttpResponse::NotFound().json(response)
    }
  }
}

#[put("/{id}")]
pub async fn update_recipe(
  db: web::Data<Database>,
  id: web::Path<i32>,
  recipe_to_update: web::Json<Recipe>,
) -> impl Responder {
  let recipe_from_json: Recipe = recipe_to_update.into_inner();
  if recipe_from_json.id == id.into_inner() {
    let result: Option<Recipe> = db.update_recipe(recipe_from_json);
    match result {
      Some(recipe) => HttpResponse::Ok().json(UpdateSuccessRecipe {
        success: true,
        recipe,
      }),
      None => HttpResponse::NotModified().json(Response {
        message: "Failed to update the Recipe".to_string(),
      }),
    }
  } else {
    HttpResponse::NotAcceptable().json(Response {
      message: "recipe id and object do not match".to_string(),
    })
  }
}

#[put("/{id}/ingredient")]
pub async fn update_recipe_ingredient(
  db: web::Data<Database>,
  id: web::Path<i32>,
  ingredient_to_udpate: web::Json<RecipeIngredient>,
) -> impl Responder {
  let ingredient_from_json: RecipeIngredient = ingredient_to_udpate.into_inner();
  if ingredient_from_json.id == id.into_inner() {
    let result: Option<RecipeIngredient> = db.update_ingredient(ingredient_from_json);
    match result {
      Some(ingredient) => HttpResponse::Ok().json(UpdateSuccessRecipeIngredient {
        success: true,
        ingredient: ingredient,
      }),
      None => HttpResponse::NotModified().json(Response {
        message: "Failed to update the ingredient".to_string(),
      }),
    }
  } else {
    HttpResponse::NotAcceptable().json(Response {
      message: "Ingredient id an id given not acceptable".to_string(),
    })
  }
}

#[put("/{id}/recipe_step")]
pub async fn update_recipe_step(
  db: web::Data<Database>,
  id: web::Path<i32>,
  recipe_step_to_update: web::Json<RecipeStep>,
) -> impl Responder {
  let recipe_step_from_json: RecipeStep = recipe_step_to_update.into_inner();
  if recipe_step_from_json.id == id.into_inner() {
    let result: Option<RecipeStep> = db.update_step(recipe_step_from_json);
    match result {
      Some(step) => HttpResponse::Ok().json(UpdateSuccessRecipeStep {
        success: true,
        recipe_step: step,
      }),
      None => HttpResponse::NotModified().json(Response {
        message: "Failed to update the recipe step".to_string(),
      }),
    }
  } else {
    HttpResponse::NotAcceptable().json(Response {
      message: "Recipe step id an id given not acceptable".to_string(),
    })
  }
}
#[delete("/{id}")]
pub async fn delete_recipe(
  db: web::Data<Database>,
  id: web::Path<i32>,
  recipe_json: web::Json<Recipe>,
) -> impl Responder {
  let recipe_to_delete = recipe_json.into_inner();
  if recipe_to_delete.id != id.into_inner() {
    HttpResponse::NotAcceptable().json(Response {
      message: "recipe id and id provided do not match".to_string(),
    })
  } else {
    let result = db.delete_recipe(recipe_to_delete);
    match result.success {
      true => HttpResponse::Ok().json(GeneralDbQuerySuccess { success: true }),
      false => HttpResponse::NotModified().json(Response {
        message: "Failed to delete the recipe".to_string(),
      }),
    }
  }
}

#[delete("/{id}/ingredient")]
pub async fn delete_recipe_ingredient(
  db: web::Data<Database>,
  id: web::Path<i32>,
  recipe_ingredient_json: web::Json<RecipeIngredient>,
) -> impl Responder {
  let recipe_ingredient = recipe_ingredient_json.into_inner();
  if recipe_ingredient.recipe_id != id.into_inner() {
    HttpResponse::NotAcceptable().json(Response {
      message: "recipe id and id provided do not match".to_string(),
    })
  } else {
    let result = db.delete_recipe_ingredient(recipe_ingredient);
    match result.success {
      true => HttpResponse::Ok().json(GeneralDbQuerySuccess { success: true }),
      false => HttpResponse::NotModified().json(Response {
        message: "Failed to delete the recipe".to_string(),
      }),
    }
  }
}

#[delete("/{id}/step")]
pub async fn delete_recipe_step(
  db: web::Data<Database>,
  id: web::Path<i32>,
  recipe_step_json: web::Json<RecipeStep>,
) -> impl Responder {
  let recipe_step = recipe_step_json.into_inner();
  if recipe_step.recipe_id != id.into_inner() {
    HttpResponse::NotAcceptable().json(Response {
      message: "recipe id and id provided do not match".to_string(),
    })
  } else {
    let result = db.delete_recipe_step(recipe_step);
    match result.success {
      true => HttpResponse::Ok().json(GeneralDbQuerySuccess { success: true }),
      false => HttpResponse::NotModified().json(Response {
        message: "Failed to delete the recipe".to_string(),
      }),
    }
  }
}
