extern crate jsonwebtoken as jwt;

use core::fmt;
use std::future::{ready, Ready};

use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, FromRequest, HttpMessage, HttpRequest};
use jsonwebtoken::{decode, encode, EncodingKey, Header, DecodingKey, Validation};
use serde::Serialize;

use chrono::{prelude::*, Duration};

use crate::models::structs::Response;
use dotenv::dotenv;


use super::structs::{UserTokenClaims, UserJwtInfo};

#[derive(Debug, Serialize)]
struct ErrorResponse {
  status: String,
  message: String,
}

impl fmt::Display for ErrorResponse {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", serde_json::to_string(&self).unwrap())
  }
}

pub struct JwtMiddleware {
  pub user_id: i32,
}

impl FromRequest for JwtMiddleware {
  type Error = ActixWebError;
  type Future = Ready<Result<Self, Self::Error>>;
  fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {

    let token = req
      .cookie("token")
      .map(|c| c.value().to_string())
      .or_else(|| {
        req
          .headers()
          .get(http::header::AUTHORIZATION)
          .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
      });
      
      if token.is_none() {
        let json_error = ErrorResponse {
          status: "fail".to_string(),
          message: "You are not logged in, please provide a token".to_string(),
        };
        return ready(Err(ErrorUnauthorized(json_error)));
      }

    let jwt_secret = std::env::var("JWT_SECRET").expect("MISSING JWT SECRET");

    let claims = match decode::<UserTokenClaims>(
      &token.unwrap(),
      &DecodingKey::from_secret(jwt_secret.as_bytes()),
      &Validation::default(),
    ) {
      Ok(c) => c.claims,
      Err(_) => {
        let json_err = ErrorResponse {
          status: "fail".to_string(),
          message: "Invalid token".to_string(),
        };
        return ready(Err(ErrorUnauthorized(json_err)));
      }
    };

    let user_id = claims.id;
    req.extensions_mut().insert::<i32>(user_id.to_owned());

    ready(Ok(JwtMiddleware { user_id }))
  }
}



// TODO check how to handle error
pub fn create_token(payload: UserJwtInfo) -> Result<String, Response> {
  dotenv().ok();

  let encoding_key = EncodingKey::from_secret(
    std::env::var("JWT_SECRET")
      .expect("MISSING JWT SECRET")
      .as_bytes(),
  );
  let UserJwtInfo {email, first_name, id, is_admin, last_name} = payload;

  let now = Utc::now();
  let iat = now.timestamp() as usize;
  let exp = (now + Duration::minutes(60)).timestamp() as usize;
  let claims = UserTokenClaims {
    id,
    email,
    first_name,
    last_name,
    is_admin,
    iat,
    exp,
  };

  let header = Header::new(jwt::Algorithm::HS256);

  
  match encode(&header, &claims, &encoding_key) {
    Ok(token) => Ok(token),
    Err(_) => Err(Response {
      message: "failed to create token".to_string(),
    }),
  }
}
