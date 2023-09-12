use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailReqs {
  pub sender: String,
  pub receiver: String,
  pub phone_number: Option<String>,
  pub body: String
} 

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailSendResult {
  pub success: bool,
  pub message: String,
}