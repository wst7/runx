
pub enum Language {
  Typescript,
  Python,
}

impl Language {
  pub fn from(language: &str) -> Self {
      match language {
          "python" | "py" => Language::Python,
          "typescript" | "ts" => Language::Typescript,
          _ => panic!("Unknown language"),
      }
  }
}
impl ToString for Language {
  fn to_string(&self) -> String {
      match self {
          Language::Python => "python".to_string(),
          Language::Typescript => "typescript".to_string(),
      }
  }
}