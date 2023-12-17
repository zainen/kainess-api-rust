use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{
  db::database::Database,
  models::structs::{KeywordFoundHerbs, Response, SearchKeywords},
};

#[get("/{last_id}")]
pub async fn get_from_herbs(db: web::Data<Database>, last_id: web::Path<i32>) -> impl Responder {
  let last_id = last_id.into_inner();
  let herb_section = db.get_herbs(last_id);
  match herb_section {
    Ok(vec) => HttpResponse::Ok().json(vec),
    Err(_) => HttpResponse::NotAcceptable().json(Response {
      message: "faiiled".to_string(),
    }),
  }
}

#[post("/")]
pub async fn search_herbs(
  db: web::Data<Database>,
  keywords_json: web::Json<SearchKeywords>,
) -> impl Responder {
  let keywords = keywords_json.into_inner();

  let results = db.search_herbs(keywords);

  match results {
    Ok(values) => HttpResponse::Ok().json(KeywordFoundHerbs { herbs: values }),
    Err(_) => HttpResponse::NotAcceptable().json(Response {
      message: "failed".to_string(),
    }),
  }
}

#[get("/search/{herb_id_path}")]
pub async fn get_herb_info(
  db: web::Data<Database>,
  herb_id_path: web::Path<i32>
) -> impl Responder {
  let herb_id = herb_id_path.into_inner();

  let query_herb = db.get_herb_information(herb_id);

  match query_herb {
    Ok(herb) => HttpResponse::Ok().json(herb),
    Err(_) => HttpResponse::NotFound().json(Response {
      message: "Herb not found".to_string()
    })
  }
}
