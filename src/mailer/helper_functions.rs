use lettre::{message::header::ContentType, Message};
use tera::Context;

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

pub fn populate_greeting_template(name: &str) {
  let mut context = Context::new();
  context.insert("name", name);
}
