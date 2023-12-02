use actix_cors::Cors;
use actix_web::{get, http::header, web, App, HttpResponse, HttpServer, Responder, Result};
use models::structs::Response;

mod api;
mod db;
mod mailer;
mod models;

#[get("/health")]
async fn healthcheck() -> impl Responder {
  let response = Response {
    message: "Everything is fine".to_string(),
  };
  HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
  let response = Response {
    message: "Resource not found".to_string(),
  };
  Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let db = db::database::Database::new();
  let app_data = web::Data::new(db);

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
      .app_data(app_data.clone())
      .configure(api::api::config)
      .service(healthcheck)
      .default_service(web::route().to(not_found))
      .wrap(actix_web::middleware::Logger::default())
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
