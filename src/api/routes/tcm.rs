use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{
  api::routes::helper_functions::herb_string_param_to_enum,
  db::database::Database,
  models::structs::{GetHerbs, KeywordFoundHerbs, Response, SearchHerbName, SearchKeywords},
};

#[get("/{last_id}")]
pub async fn get_from_herbs(db: web::Data<Database>, last_id: web::Path<i32>) -> impl Responder {
  // TODO MAKE TWO END POINTS TO SEPARATE HERB LIST FROM PAGES VEC
  let last_id = last_id.into_inner();
  let herb_section = db.get_herbs(last_id);
  let pages = db.get_herb_count();
  match herb_section {
    Ok(vec) => HttpResponse::Ok().json(GetHerbs { herbs: vec, pages }),
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

#[post("/search/name")]
pub async fn search_herb_name(
  db: web::Data<Database>,
  search_name_params_path: web::Json<SearchHerbName>,
) -> impl Responder {
  let SearchHerbName {
    language,
    herb_name,
  } = search_name_params_path.into_inner();
  let search_language = match herb_string_param_to_enum(&language) {
    Some(search_by) => search_by,
    None => {
      return HttpResponse::NotAcceptable().json(Response {
        message: "Unsupported language".to_string(),
      })
    }
  };
  let herbs = db.search_herb_name_en(herb_name, search_language);

  match herbs {
    Ok(herbs) => HttpResponse::Ok().json(herbs),
    Err(_) => HttpResponse::BadRequest().json(Response {
      message: "DB query failed".to_string(),
    }),
  }
}
