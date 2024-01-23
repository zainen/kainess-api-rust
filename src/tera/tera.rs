use tera::Tera;

#[derive(Debug)]
pub struct TemplatesConsumer {
  pub tera: Tera,
}

impl TemplatesConsumer {
  pub fn new() -> Self {
    // Create a new Tera instance and add a template from a string
    let dir = std::env::current_dir().unwrap().join("templates/*");
    let dir_str = dir.to_str().unwrap();
    println!("{:?}", dir_str);
    let mut tera = Tera::new(dir_str).unwrap();

    tera
      .add_raw_template("hello", "Hello, {{ name }}!")
      .unwrap();
    // Prepare the context with some data
    let mut context = tera::Context::new();
    context.insert("name", "World");

    // Render the template with the given context
    let rendered = tera.render("hello", &context).unwrap();
    assert_eq!(rendered, "Hello, World!");

    let mut context = tera::Context::new();
    context.insert("name", "World");
    let render = tera.render("greeting.txt", &context);
    println!("{:?}", render);

    TemplatesConsumer { tera }
  }
}
