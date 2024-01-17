use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{
  api::routes::helper_functions::validate_query_type,
  db::database::Database,
  models::structs::{GetHerbs, KeywordFoundHerbs, Response, SearchQueryParams, SearchKeywords},
};

// STARTING FROM PAGE 1 CHANGE TO INDEX 0
#[get("/{page_number}")]
pub async fn get_from_herbs(db: web::Data<Database>, page_number: web::Path<usize>) -> impl Responder {
  // TODO MAKE TWO END POINTS TO SEPARATE HERB LIST FROM PAGES VEC
  let page_number = page_number.into_inner();
  let pages = db.get_herb_count();
  
  // println!("{:?}", db.unique_meridians().unwrap());

  match pages.len() < page_number || page_number < 1 {
    true => {
      return HttpResponse::NotAcceptable().json(Response {
      message: "faiiled".to_string(),
      })
    },
    false => {}
  };


  let page_number_to_index = page_number - 1;


  // TODO HANDLE POSSIBLE PAGE EXHAUSTION
  let page_index = pages[page_number_to_index];
  let herb_section = db.get_herbs_limit(page_index);
  match herb_section {
    Ok(vec) => HttpResponse::Ok().json(GetHerbs { herbs: vec, pages }),
    Err(_) => HttpResponse::NotAcceptable().json(Response {
      message: "faiiled".to_string(),
    }),
  }
}

#[post("/")]
pub async fn search_herbs_keywords(
  db: web::Data<Database>,
  keywords_json: web::Json<SearchKeywords>,
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

#[post("/search/query")]
pub async fn search_herbs_temp(
  db: web::Data<Database>,
  search_name_params_path: web::Json<SearchQueryParams>,
) -> impl Responder {
  let SearchQueryParams {
    query_type,
    params,
  } = search_name_params_path.into_inner();
  if let None = validate_query_type(&query_type) {
    return HttpResponse::NotAcceptable().json(Response {
      message: "Unsupported language".to_string(),
    })
  }
  let herbs = db.search_herbs_with_params(params);

  match herbs {
    Ok(herbs) => HttpResponse::Ok().json(herbs),
    Err(_) => HttpResponse::BadRequest().json(Response {
      message: "DB query failed".to_string(),
    }),
  }
}
