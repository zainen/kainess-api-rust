use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};

// use serde::{Deserialize, Serialize};

// use reqwest::Client as HttpClient;

// use async_trait::async_trait;s

// mod error_handler;
// mod recipie;
mod models;
mod db;

async fn hello() -> impl Responder {
  HttpResponse::Ok().body("HELLO")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // use self::db::schema::recipie::dsl::*;

  // let connection = &mut establish_connection();

  HttpServer::new(move || {
    App::new()
      .wrap(
        Cors::permissive()
          .allowed_origin_fn(|origin, _request_head| {
            // CHECK add possible origins
            origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
          })
          .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
          .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
          .allowed_header(header::CONTENT_TYPE)
          .supports_credentials()
          // CHECK AGE
          .max_age(3600),
      )
      .app_data(())
      .route("/hello", web::get().to(hello))
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
