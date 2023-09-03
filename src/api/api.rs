use actix_web::{web};

use super::routes::recipe::{get_recipes, get_recipe_details, create_recipe};


pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/recipe")
      .service(get_recipes)
      .service(get_recipe_details)
      .service(create_recipe)
  );
}
