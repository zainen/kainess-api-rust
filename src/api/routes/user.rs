use crate::{
  api::auth::{
    jwt_auth::{self, create_token},
    structs::LoginParams,
  },
  models::structs::Response,
};
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};

use crate::{db::database::Database, models::structs::UserValidationParams};

extern crate jsonwebtoken as jwt;

#[post("/create-user")]
pub async fn create_user(
  db: web::Data<Database>,
  params_json: web::Json<UserValidationParams>,
) -> impl Responder {
  let params: UserValidationParams = params_json.into_inner();

  match db.create_user(params) {
    Ok(payload) => match create_token(payload) {
      Ok(token) => HttpResponse::Ok().json(token),
      Err(e) => HttpResponse::BadRequest().json(e),
    },
    // left default response as static Response to hide reason for db failure.
    // TODO check other way of printing error
    Err(e) => {
      eprintln!("{e}");
      HttpResponse::BadRequest().json(Response {
        message: "Failed to create user".to_string(),
      })
    }
  }
}

#[post("/")]
pub async fn login(db: web::Data<Database>, params_json: web::Json<LoginParams>) -> impl Responder {
  let creds = params_json.into_inner();
  match db.check_user(creds) {
    Ok(payload) => match create_token(payload) {
      Ok(token) => HttpResponse::Ok().json(token),
      Err(e) => HttpResponse::BadRequest().json(e),
    },
    Err(e) => HttpResponse::BadRequest().json(e),
  }
}

#[get("/test")]
pub async fn test(req: HttpRequest, _: jwt_auth::JwtMiddleware) -> impl Responder {
  let _ext = req.extensions();

  HttpResponse::Ok().json(Response {
    message: "PASS TOKEN VALIDATION".to_string(),
  })
}
