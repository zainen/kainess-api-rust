use crate::models::structs::Response;
use actix_web::{post, web, HttpResponse, Responder};

use crate::{
  db::database::Database,
  models::structs::{GeneralDbQuerySuccess, UserValidationParams},
};

#[post("/")]
pub async fn login(db: web::Data<Database>, params_json: web::Json<UserValidationParams>) -> impl Responder {
  let user_params = params_json.into_inner();

  let creds = UserValidationParams {
    email: user_params.email,
    password: user_params.password,
  };
  let res = db.check_user(creds);
  HttpResponse::Ok().json(GeneralDbQuerySuccess { success: res })
}

#[post("/create-user")]
pub async fn create_user(
  db: web::Data<Database>,
  params_json: web::Json<UserValidationParams>,
) -> impl Responder {
  let params = params_json.into_inner();
  println!("start user creation");

  match db.create_user(params) {
    Ok(user) => HttpResponse::Ok().json(user),
    Err(e) => {
      println!("{:?}", e);
      HttpResponse::BadRequest().json(Response {
        message: "Failed tot create user".to_string(),
      })
    }
  }
}
