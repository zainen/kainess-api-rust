use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailReqs {
  pub sender: String,
  pub receiver: String,
  pub first_name: String,
  pub last_name: String,
  pub phone_number: Option<String>,
  pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailSendResult {
  pub success: bool,
  pub message: String,
}
