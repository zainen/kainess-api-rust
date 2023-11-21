use crate::{models::structs::Response, user::helper_fucntions::create_token};
use actix_web::{post, web, HttpResponse, Responder};

use crate::{
  db::database::Database,
  models::structs::UserValidationParams,
};

extern crate jsonwebtoken as jwt;

#[post("/create-user")]
pub async fn create_user(
  db: web::Data<Database>,
  params_json: web::Json<UserValidationParams>,
) -> impl Responder {
  let params = params_json.into_inner();
  
  match db.create_user(params) {
    Ok(payload) => {
      match create_token(&payload) {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(e) => HttpResponse::BadRequest().json(e)
        
      }
    },
    // TODO handle unique errors
    Err(_) => {
      HttpResponse::BadRequest().json(Response {
        message: "Failed to create user".to_string(),
      })
    }
  }
}


#[post("/")]
pub async fn login(db: web::Data<Database>, params_json: web::Json<UserValidationParams>) -> impl Responder {
  let creds = params_json.into_inner();
  match db.check_user(creds) {
    Ok(payload) => {
      match create_token(&payload) {
        Ok(token) => HttpResponse::Ok().json( token ),
        Err(e) => HttpResponse::BadRequest().json(e)
      }
    },
    Err(e) => {
      HttpResponse::BadRequest().json(e)
    }
  }

}