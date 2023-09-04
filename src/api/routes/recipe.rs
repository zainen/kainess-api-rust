use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use crate::models::structs::{CreateRecipe, Response, RecipeWithDetails, UpdateSuccessRecipe};
use crate::{
  db::database::Database,
  models::structs::{NewIngredient, NewRecipe, NewStep, Recipe, RecipeIngredient, RecipeStep},
};
use crate::api::routes::helper_functions::db_update_recipe;

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
  let ingredients_to_insert: Vec<NewIngredient> = inner_info.ingredients;
  let steps_to_insert: Vec<NewStep> = inner_info.steps;

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
pub async fn update_recipe(db: web::Data<Database>, id: web::Path<i32>, recipe_to_update: web::Json<Recipe>) -> impl Responder {
  let recipe_from_json: Recipe = recipe_to_update.into_inner();
  if recipe_from_json.id == id.into_inner() {
    let result: Option<Recipe> = db_update_recipe(db, recipe_from_json).await;
    match result {
      Some(recipe) => {
        HttpResponse::Ok().json(UpdateSuccessRecipe {
          success: true,
          recipe
        })
      },
      None => {
        HttpResponse::NotModified().json(Response {
          message: "Failed to update the Recipe".to_string()
        })
      }
    }
  } else {

    HttpResponse::NotAcceptable().json(Response {
      message: "recipe id and object do not match".to_string()
    })
  }
}

#[put("/{id}/ingredient")]
pub async fn update_recipe_ingredient(db: web::Data<Database>, id: web::Path<i32>, ingredient_to_udpate: web::Json<RecipeIngredient>) -> impl Responder {
  let ingredient_from_json: RecipeIngredient = ingredient_to_udpate.into_inner();
  HttpResponse::Ok()
}
