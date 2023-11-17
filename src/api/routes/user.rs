use crate::models::structs::Response;
use actix_web::{post, web, HttpResponse, Responder};

use crate::{
  db::database::Database,
  models::structs::{GeneralDbQuerySuccess, UserValidationParams},
};
use dotenv::dotenv;

extern crate jsonwebtoken as jwt;
use jwt::{decode, encode, Header, Validation, EncodingKey};

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


#[post("/")]
pub async fn login(db: web::Data<Database>, params_json: web::Json<UserValidationParams>) -> impl Responder {
  dotenv().ok();
  let secret = std::env::var("JWT_SECRET").expect("MISSING JWT SECRET");
  let creds = params_json.into_inner();
  match db.check_user(creds) {
    Ok(payload) => {
      let encoding_key = EncodingKey::from_secret(secret.as_bytes());
    
      let header = Header::new(jwt::Algorithm::HS256);
      let token = encode(&header, &payload, &encoding_key).unwrap();
    
      HttpResponse::Ok().json( token )
    },
    Err(e) => {
      HttpResponse::BadRequest().json(e)
    }
  }

}