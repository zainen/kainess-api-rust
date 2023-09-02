use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use crate::{
  db::database::Database,
  models::structs::{NewIngredient, NewRecipe, NewStep, Recipe, RecipeIngredient, RecipeStep},
};

#[get("/hello")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("HELLO")
}

#[get("/")]
async fn get_recipes(db: web::Data<Database>) -> impl Responder {
  let recipes = db.get_recipes();
  HttpResponse::Ok().json(recipes)
}

#[get("/{id}")]
async fn get_recipe_details(db: web::Data<Database>, id: web::Path<i32>) -> impl Responder {
  let recipe_with_details = db.get_recipe_details(*id);
  HttpResponse::Ok().json(recipe_with_details)
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/recipe")
      .service(hello)
      .service(get_recipes)
      .service(get_recipe_details),
  );
}
