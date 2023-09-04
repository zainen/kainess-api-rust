use actix_web::web;

use super::routes::recipe::{create_recipe, get_recipe_details, get_recipes};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/recipe")
      .service(get_recipes)
      .service(get_recipe_details)
      .service(create_recipe),
  );
}
