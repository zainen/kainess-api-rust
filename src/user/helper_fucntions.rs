extern crate jsonwebtoken as jwt;
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::models::structs::{UserJwtInfo, Response};
use dotenv::dotenv;

// TODO check how to handle error
pub fn create_token (payload: &UserJwtInfo) -> Result<String, Response> {
  dotenv().ok();

  let encoding_key = EncodingKey::from_secret(std::env::var("JWT_SECRET").expect("MISSING JWT SECRET").as_bytes());
  
  let header = Header::new(jwt::Algorithm::HS256);
  match encode(&header, &payload, &encoding_key) {
    Ok(token) => Ok(token),
    Err(_) => Err(Response {
      message: "failed to create token".to_string()
    })
  }
}