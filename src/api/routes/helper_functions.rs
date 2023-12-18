use crate::models::structs::SearchBy;

pub fn herb_string_param_to_enum(str: &str) -> Option<SearchBy> {
  match str {
    "english" => Some(SearchBy::English),
    "chinese" => Some(SearchBy::Chinese),
    _ => None
  }
}