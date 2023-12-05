use serde::{Deserialize, Serialize};
use diesel::{AsChangeset, Identifiable};


#[derive(Debug, Deserialize, Serialize)]
pub struct UserTokenClaims {
  pub id: i32,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub is_admin: bool,
  pub iat: usize,
  pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct LoginParams {
  pub email: String,
  pub password: String,
}

#[derive(Serialize, Deserialize, Identifiable, Debug, AsChangeset)]
#[diesel(table_name = crate::models::schema::users)]
pub struct UserJwtInfo {
  pub id: i32,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub is_admin: bool,
}
