use crate::models::structs::SearchBy;

pub fn validate_query_type(query_type: &str) -> Option<SearchBy> {
  // TODO CHANGE BASED ON QUERY OPTIONS
  
  match query_type {
    "english" => Some(SearchBy::English),
    "chinese" => Some(SearchBy::Chinese),
    _ => None,
  }
}
