use lettre::{
  message::header::ContentType,
  transport::smtp::{authentication::Credentials, response::Response, Error},
  Message, SmtpTransport, Transport,
};
use tera::{Context, Tera};

pub struct Mailer {
  pub tera: Tera,
}

impl Mailer {
  pub fn new() -> Self {
    let dir = std::env::current_dir().unwrap().join("templates/**/*");
    let dir_str = dir.to_str().unwrap();
    println!("{:?}", dir_str);
    let tera = Tera::new(dir_str).unwrap();

    Mailer { tera }
  }

  pub fn render_template(&mut self, template: &str, context: &Context) -> String {
    self.tera.render(template, context).unwrap()
  }

  pub fn create_message(
    &self,
    sender: &str,
    reciever: &str,
    subject: &str,
    template: String,
    content_type: ContentType,
  ) -> Message {
    Message::builder()
      .from(sender.parse().unwrap())
      .reply_to(sender.parse().unwrap())
      .to(reciever.parse().unwrap())
      .subject(subject)
      .header(content_type)
      .body(template)
      .unwrap()
  }

  pub fn send(&self, message: Message) -> Result<Response, Error> {
    let creds = Credentials::new(
      std::env::var("GMAIL_USER").expect("Missing email user"),
      std::env::var("GMAIL_PASS").expect("Missing email user password"),
    );

    // Open a remote connection to gmail
    let sender = SmtpTransport::relay("smtp.gmail.com")
      .unwrap()
      .credentials(creds)
      .build();

    sender.send(&message)
  }
}
