use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use crate::models::structs::{GeneralDbQuerySuccess, UpdateSuccessRecipeStep};
use crate::{
  api::auth::jwt_auth,
  models::structs::{
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
  // visibile to without authentication
  let recipes: Vec<Recipe> = db.get_recipes();
  HttpResponse::Ok().json(recipes)
}

#[post("/")]
pub async fn create_recipe(
  // visibile without authentication
  db: web::Data<Database>,
  recipe_information: web::Json<CreateRecipe>,
  jwt: jwt_auth::JwtMiddleware,
) -> impl Responder {
  // extract web json
  let inner_info: CreateRecipe = recipe_information.into_inner();
  let recipe_to_create: NewRecipe = inner_info.recipe;

  // JWT block if not correct id
  if &jwt.claims.id != &recipe_to_create.creator_id {
    return HttpResponse::NotAcceptable().json(Response {
      message: "Id does not match".to_string(),
    });
  }
  let ingredients_to_insert: Vec<NewRecipeIngredient> = inner_info.ingredients;
  let steps_to_insert: Vec<NewRecipeStep> = inner_info.steps;

  let insert_result = db.create_recipe(recipe_to_create, ingredients_to_insert, steps_to_insert);

  match insert_result {
    Ok(_) => HttpResponse::Ok().json(GeneralDbQuerySuccess { success: true }),
    Err(_) => HttpResponse::NotAcceptable().json(GeneralDbQuerySuccess { success: false }),
  }
}

#[get("/{recipe_id_path}")]
pub async fn get_recipe_details(
  db: web::Data<Database>,
  recipe_id_path: web::Path<i32>,
) -> impl Responder {
  // extract web json
  let recipe_id = recipe_id_path.into_inner();

  let recipe_with_details: RecipeWithDetails = db.get_recipe_details(recipe_id);
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

#[put("/{recipe_id_path}/base")]
pub async fn update_recipe_base(
  db: web::Data<Database>,
  recipe_id_path: web::Path<i32>,
  recipe_to_update_json: web::Json<Recipe>,
  jwt: jwt_auth::JwtMiddleware,
) -> impl Responder {
  // extract web json
  let recipe_to_update: Recipe = recipe_to_update_json.into_inner();

  // block if not right id
  if &jwt.claims.id != &recipe_to_update.creator_id {
    return HttpResponse::NotAcceptable().json(Response {
      message: "Id does not match".to_string(),
    });
  }
  // check recipe base id and attempt update
  if recipe_to_update.id == recipe_id_path.into_inner() {
    let result: Option<Recipe> = db.update_recipe(recipe_to_update);
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

#[put("/{recipe_id_path}/ingredient/{ingredient_id_path}")]
pub async fn update_recipe_ingredient(
  db: web::Data<Database>,
  path_params: web::Path<(i32, i32)>,
  ingredient_json: web::Json<RecipeIngredient>,
  jwt: jwt_auth::JwtMiddleware,
) -> impl Responder {
  // extract web json
  let (recipe_id, ingredient_id) = path_params.into_inner();
  let ingredient: RecipeIngredient = ingredient_json.into_inner();

  if let Some(recipe) = db.get_recipe(recipe_id) {
    if recipe.creator_id != jwt.claims.id {
      return HttpResponse::NotAcceptable().json(Response {
        message: "Not Permitted".to_string(),
      });
    }
  }

  let ingredient_id_check = ingredient.id != ingredient_id;
  if ingredient_id_check {
    return HttpResponse::NotAcceptable().json(Response {
      message: "Ingridient id does not match path".to_string(),
    });
  }
  if ingredient.recipe_id == recipe_id {
    let result: Option<RecipeIngredient> = db.update_ingredient(ingredient);
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

#[put("/{recipe_id_path}/step/{step_id_path}")]
pub async fn update_recipe_step(
  db: web::Data<Database>,
  path_params: web::Path<(i32, i32)>,
  recipe_step_json: web::Json<RecipeStep>,
  jwt: jwt_auth::JwtMiddleware,
) -> impl Responder {
  let (recipe_id, step_id) = path_params.into_inner();
  if let Some(recipe) = db.get_recipe(recipe_id) {
    if recipe.creator_id != jwt.claims.id {
      return HttpResponse::NotAcceptable().json(Response {
        message: "Not Permitted".to_string(),
      });
    }
  }

  let recipe_step = recipe_step_json.into_inner();
  let step_id_check = recipe_step.id != step_id;
  if step_id_check {
    return HttpResponse::NotAcceptable().json(Response {
      message: "Step id does not match path".to_string(),
    });
  }
  if recipe_step.recipe_id == recipe_id {
    let result: Option<RecipeStep> = db.update_step(recipe_step);
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

#[post("/{recipe_id_path}/step/{step_number_to_add}")]
pub async fn add_recipe_step(
  db: web::Data<Database>,
  path_params: web::Path<(i32, i32)>,
  recipe_step_json: web::Json<NewRecipeStep>,
  jwt: jwt_auth::JwtMiddleware,
) -> impl Responder {
  // extract web json
  let (recipe_id, step_number) = path_params.into_inner();
  let recipe_step = recipe_step_json.into_inner();

  if let Some(recipe) = db.get_recipe(recipe_id) {
    if recipe.creator_id != jwt.claims.id {
      return HttpResponse::NotAcceptable().json(Response {
        message: "Not Permitted".to_string(),
      });
    }
  }

  let updated_ingredients = db.add_recipe_step(recipe_step, recipe_id, step_number - 1);
  HttpResponse::Ok().json(updated_ingredients)
}

#[delete("/{recipe_id_path}")]
pub async fn delete_recipe(
  db: web::Data<Database>,
  recipe_id_path: web::Path<i32>,
  recipe_json: web::Json<Recipe>,
  jwt: jwt_auth::JwtMiddleware,
) -> impl Responder {
  // extract web json
  let recipe_to_delete = recipe_json.into_inner();
  // reject if dont match
  if recipe_to_delete.id != recipe_id_path.into_inner() {
    return HttpResponse::NotAcceptable().json(Response {
      message: "recipe id and id provided do not match".to_string(),
    });
  }
  // reject if not recipe owner
  if let Some(recipe) = db.get_recipe(recipe_to_delete.id) {
    if recipe.creator_id != jwt.claims.id {
      return HttpResponse::NotAcceptable().json(Response {
        message: "Not Permitted".to_string(),
      });
    }
  }
  let result = db.delete_recipe(recipe_to_delete);
  match result.success {
    true => HttpResponse::Ok().json(GeneralDbQuerySuccess { success: true }),
    false => HttpResponse::NotModified().json(Response {
      message: "Failed to delete the recipe".to_string(),
    }),
  }
}

#[delete("/{recipe_id_path}/ingredient/{ingredient_id_path}")]
pub async fn delete_recipe_ingredient(
  db: web::Data<Database>,
  path_params: web::Path<(i32, i32)>,
  ingredient_json: web::Json<RecipeIngredient>,
  jwt: jwt_auth::JwtMiddleware,
) -> impl Responder {
  // extract web json
  let (recipe_id, ingredient_id) = path_params.into_inner();
  let ingredient = ingredient_json.into_inner();

  let ingredient_id_check_failed = ingredient_id != ingredient.id;
  if ingredient_id_check_failed {
    return HttpResponse::NotAcceptable().json(Response {
      message: "ingredient id does not match".to_string(),
    });
  }
  if ingredient.recipe_id != recipe_id {
    return HttpResponse::NotAcceptable().json(Response {
      message: "recipe id and id provided do not match".to_string(),
    });
  }
  // reject if not recipe owner
  if let Some(recipe) = db.get_recipe(recipe_id) {
    if recipe.creator_id != jwt.claims.id {
      return HttpResponse::NotAcceptable().json(Response {
        message: "Not Permitted".to_string(),
      });
    }
  }
  let result = db.delete_recipe_ingredient(ingredient);
  match result.success {
    true => HttpResponse::Ok().json(GeneralDbQuerySuccess { success: true }),
    false => HttpResponse::NotModified().json(Response {
      message: "Failed to delete the recipe".to_string(),
    }),
  }
}

#[delete("/{recipe_id_path}/step/{step_id_path}")]
pub async fn delete_recipe_step(
  db: web::Data<Database>,
  path_params: web::Path<(i32, i32)>,
  recipe_step_json: web::Json<RecipeStep>,
  jwt: jwt_auth::JwtMiddleware,
) -> impl Responder {
  // extract web json
  let (recipe_id, step_id) = path_params.into_inner();
  let recipe_step = recipe_step_json.into_inner();

  let step_id_check = step_id != recipe_step.id;
  if step_id_check {
    return HttpResponse::NotAcceptable().json(Response {
      message: "step id does not match".to_string(),
    });
  }
  if recipe_step.recipe_id != recipe_id {
    return HttpResponse::NotAcceptable().json(Response {
      message: "recipe id and id provided do not match".to_string(),
    });
  }
  // reject if not recipe owner
  if let Some(recipe) = db.get_recipe(recipe_id) {
    if recipe.creator_id != jwt.claims.id {
      return HttpResponse::NotAcceptable().json(Response {
        message: "Not Permitted".to_string(),
      });
    }
  }
  let result = db.delete_recipe_step(recipe_step);
  match result.success {
    true => HttpResponse::Ok().json(GeneralDbQuerySuccess { success: true }),
    false => HttpResponse::NotModified().json(Response {
      message: "Failed to delete the recipe".to_string(),
    }),
  }
}
