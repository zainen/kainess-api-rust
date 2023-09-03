use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use crate::{
  db::database::Database,
  models::structs::{NewIngredient, NewRecipe, NewStep, Recipe, RecipeIngredient, RecipeStep},
};

use crate::api::routes::recipe::{get_recipes, get_recipe_details};

#[get("/hello")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("HELLO")
}





pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/recipe")
      .service(hello)
      .service(get_recipes)
      .service(get_recipe_details),
  );
}
