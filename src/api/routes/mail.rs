use actix_web::{post, web, Responder, HttpResponse};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use dotenv::dotenv;

use crate::{db::database::Database, mailer::{structs::{EmailReqs, EmailSendResult}, helper_functions::create_message}};

#[post("/")]
pub async fn handle_email(_db: web::Data<Database>, params_json: web::Json<EmailReqs>) -> impl Responder {
dotenv().ok();
let EmailReqs {sender, receiver, phone_number, body} = params_json.into_inner();
let subject = match phone_number {
  Some(pn) => format!("Kainess incoming inquiry with phone number {}", pn),
  None => "Kainess incoming inquiry".to_string()
};
let email = create_message(&sender, &receiver, &subject, &body);

let creds = Credentials::new(std::env::var("GMAIL_USER").expect("Missing email user"), std::env::var("GMAIL_PASS").expect("Missing email user password"));

// Open a remote connection to gmail
let mailer = SmtpTransport::relay("smtp.gmail.com")
    .unwrap()
    .credentials(creds)
    .build();

// Send the email
match mailer.send(&email) {
      Ok(_) => HttpResponse::Ok().json(EmailSendResult {
        success: true,
        message: "Email sent".to_string(),
      }),
    Err(e) => {
      println!("{:?}", e);
      HttpResponse::NotAcceptable().json(EmailSendResult {
        success: false,
        message: "Failed to send".to_string()
      })
  }
}
}
