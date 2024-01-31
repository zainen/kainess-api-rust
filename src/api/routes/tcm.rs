use actix_web::{get, web, HttpResponse, Responder};

use crate::{
  db::database::Database,
  models::structs::{GetHerbs, KeywordFoundHerbs, Response, ResponseMeridians, SearchMeridians},
};

// STARTING FROM PAGE 1 CHANGE TO INDEX 0
#[get("/{page_number}")]
pub async fn get_from_herbs(
  db: web::Data<Database>,
  page_number: web::Path<usize>,
) -> impl Responder {
  // TODO MAKE TWO END POINTS TO SEPARATE HERB LIST FROM PAGES VEC
  let page_number = page_number.into_inner();
  let pages = db.get_herb_count();

  if pages.len() < page_number || page_number < 1 {
    return HttpResponse::NotAcceptable().json(Response {
      message: "faiiled".to_string(),
    })
  };

  let page_number_to_index = page_number - 1;

  let page_index = pages[page_number_to_index];
  let herb_section = db.get_herbs_limit(page_index);
  match herb_section {
    Ok(vec) => HttpResponse::Ok().json(GetHerbs { herbs: vec, pages }),
    Err(_) => HttpResponse::NotAcceptable().json(Response {
      message: "faiiled".to_string(),
    }),
  }
}

#[get("/search/keywords-filter")]
pub async fn search_herbs_keywords(
  db: web::Data<Database>,
  keywords_json: web::Json<SearchMeridians>,
) -> impl Responder {
  let keywords = keywords_json.into_inner();

  let results = db.search_herbs_keywords(keywords);

  match results {
    Ok(values) => HttpResponse::Ok().json(KeywordFoundHerbs { herbs: values }),
    Err(_) => HttpResponse::NotAcceptable().json(Response {
      message: "failed".to_string(),
    }),
  }
}

#[get("/herbs/{herb_id_path}")]
pub async fn get_herb_info(
  db: web::Data<Database>,
  herb_id_path: web::Path<i32>,
) -> impl Responder {
  let herb_id = herb_id_path.into_inner();

  let query_herb = db.get_herb_information(herb_id);

  match query_herb {
    Ok(herb) => HttpResponse::Ok().json(herb),
    Err(_) => HttpResponse::NotFound().json(Response {
      message: "Herb not found".to_string(),
    }),
  }
}

#[get("/search/meridians")]
pub async fn get_meridian_options(db: web::Data<Database>) -> impl Responder {
  let unique_meridians = db.unique_meridians().expect("DB meridians Query Failed");

  HttpResponse::Ok().json(ResponseMeridians {
    meridians: unique_meridians,
  })
}
