use actix_web::web;

use super::routes::{
  mail::handle_email,
  recipe::{
    add_recipe_step, create_recipe, delete_recipe, delete_recipe_ingredient, delete_recipe_step,
    get_recipe_details, get_recipes, update_recipe_base, update_recipe_ingredient,
    update_recipe_step,
  },
  tcm::{get_from_herbs, get_herb_info, search_herbs_temp, search_herbs},
  user::{create_user, login, test},
};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .service(
      web::scope("/recipe")
        .service(get_recipes)
        .service(get_recipe_details)
        .service(create_recipe)
        .service(update_recipe_base)
        .service(update_recipe_ingredient)
        .service(update_recipe_step)
        .service(add_recipe_step)
        .service(delete_recipe)
        .service(delete_recipe_ingredient)
        .service(delete_recipe_step),
    )
    .service(web::scope("/send-email").service(handle_email))
    .service(
      web::scope("/user")
        .service(login)
        .service(create_user)
        .service(test),
    )
    .service(
      web::scope("/tcm")
        .service(get_from_herbs)
        .service(search_herbs)
        .service(get_herb_info)
        .service(search_herbs_temp),
    );
}
