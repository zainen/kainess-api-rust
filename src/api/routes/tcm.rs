use actix_web::{get, web, HttpResponse, Responder};

use crate::{
  db::database::Database,
  models::structs::{Herb, Response},
};

#[get("/")]
pub async fn get_from_herbs(db: web::Data<Database>) -> impl Responder {
  let dummy = db.get_herbs(Some("here".to_string()));
  match dummy {
    Ok(vec) => HttpResponse::Ok().json(vec),
    Err(_) => HttpResponse::NotAcceptable().json(Response {
      message: "faiiled".to_string(),
    }),
  }
}
