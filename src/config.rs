#[derive(Debug, Clone)]
pub struct Config {
  pub database_url: String,
  pub jwt_secret: String,
  pub jwt_expires_in: String,
  pub jwt_maxage: i32,
}

impl Config {
  pub fn init() -> Config {
    let database_url = std::env::var("DATABASE_URL").expect("DB URL MISSING");
    let jwt_secret = std::env::var("JWT_SECRET").expect("MISSING JWT SECRET");
    let jwt_expires_in = std::env::var("JWT_SECRET").expect("MISSING JWT EXPIRY");
    let jwt_maxage = std::env::var("JWT_MAXAGE").expect("MISSING MAX AGE");

    Config {
      database_url,
      jwt_secret,
      jwt_expires_in,
      jwt_maxage: jwt_maxage.parse<i32>().unwrap()
    }
  }
}