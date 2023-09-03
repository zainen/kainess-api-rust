use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use crate::models::structs::{Response, CreateRecipe};
use crate::{
  db::database::Database,
  models::structs::{NewIngredient, NewRecipe, NewStep, Recipe, RecipeIngredient, RecipeStep},
};

#[get("/")]
pub async fn get_recipes(db: web::Data<Database>) -> impl Responder {
  let recipes = db.get_recipes();
  HttpResponse::Ok().json(recipes)
}

#[get("/{id}")]
pub async fn get_recipe_details(db: web::Data<Database>, id: web::Path<i32>) -> impl Responder {
  let recipe_with_details = db.get_recipe_details(*id);
  match &recipe_with_details.recipe {
    Some(_) => HttpResponse::Ok().json(recipe_with_details),
    None => { 
      let response = Response { message: "Recipe Not Found".to_string()};
      HttpResponse::NotFound().json(response)
    }
  }
}

#[post("/")]
pub async fn create_recipe(db: web::Data<Database>, recipe_information: web::Json<CreateRecipe>) -> impl Responder {
  let inner_info = recipe_information.into_inner();
  let recipe_to_create = inner_info.recipe;
  let ingredients_to_insert = inner_info.ingredients;
  let steps_to_insert = inner_info.steps;

  let insert_result = db.create_recipe(recipe_to_create, ingredients_to_insert, steps_to_insert);

  match insert_result {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::NotAcceptable()
  }
}