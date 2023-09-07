use actix_web::web;

use super::routes::recipe::{
  create_recipe, delete_recipe, delete_recipe_ingredient, delete_recipe_step, get_recipe_details,
  get_recipes, update_recipe,
};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/recipe")
      .service(get_recipes)
      .service(get_recipe_details)
      .service(create_recipe)
      .service(update_recipe)
      .service(delete_recipe)
      .service(delete_recipe_ingredient)
      .service(delete_recipe_step),
  );
}
