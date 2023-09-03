use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use crate::models::structs::Response;
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