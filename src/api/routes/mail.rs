use actix_web::{post, web, Responder, HttpResponse};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use dotenv::dotenv;

use crate::{db::database::Database, mailer::structs::{EmailReqs, EmailSendResult}};

#[post("/")]
pub async fn handle_email(_db: web::Data<Database>, params_json: web::Json<EmailReqs>) -> impl Responder {
dotenv().ok();
let EmailReqs {sender, receiver, subject, body} = params_json.into_inner();
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

pub fn create_message(sender: &str, reciever: &str, subject: &str, body: &str) -> Message {
  let email = Message::builder()
    .from(sender.parse().unwrap())
    .reply_to(sender.parse().unwrap())
    .to(reciever.parse().unwrap())
    .subject(subject)
    .header(ContentType::TEXT_PLAIN)
    .body(String::from(body))
    .unwrap();

  email
}