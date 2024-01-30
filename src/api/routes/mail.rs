use actix_web::{get, post, web, HttpResponse, Responder};
use lettre::message::header::ContentType;

use crate::{
  db::database::Database,
  mailer::{
    mailer::Mailer,
    structs::{EmailReqs, EmailSendResult},
  },
};

// TODO REFACTOR FULLY FOR TEMPLATES
#[post("/")]
pub async fn handle_email(
  _db: web::Data<Database>,
  params_json: web::Json<EmailReqs>,
) -> impl Responder {
  let EmailReqs {
    first_name,
    last_name,
    sender,
    receiver,
    phone_number,
    body,
  } = params_json.into_inner();
  let subject = match phone_number {
    Some(phone_number) => format!(
      "Kainess incoming inquiry from {} {} with phone number {}",
      first_name, last_name, phone_number
    ),
    None => "Kainess incoming inquiry".to_string(),
  };
  let mailer = Mailer::new();
  // TODO not worred about this as a template will be used 
  let email = mailer.create_message(&sender, &receiver, &subject, body, ContentType::TEXT_PLAIN);

  match mailer.send(email) {
    Ok(_) => HttpResponse::Ok().json(EmailSendResult {
      success: true,
      message: "Email sent".to_string(),
    }),
    Err(e) => {
      eprintln!("{e}");
      HttpResponse::NotAcceptable().json(EmailSendResult {
        success: false,
        message: "Failed to send".to_string(),
      })
    }
  }
}

#[get("/")]
pub async fn try_template(
  _db: web::Data<Database>,
) -> impl Responder {
  let mut context = tera::Context::new();
  context.insert("name", "Zainen");
  let mut mailer = Mailer::new();
  let rendered = &mailer.render_template("index.html", &context);
  let message = mailer.create_message(
    "zainen.test@gmail.com",
    "zainen.test@gmail.com",
    "Hello",
    rendered.to_string(),
    ContentType::TEXT_HTML,
  );
  match mailer.send(message) {
    Ok(_) => HttpResponse::Ok().json(EmailSendResult {
      success: true,
      message: "Email sent".to_string(),
    }),
    Err(e) => {
      eprintln!("{e}");
      HttpResponse::NotAcceptable().json(EmailSendResult {
        success: false,
        message: "Failed to send".to_string(),
      })
    }
  }
}
