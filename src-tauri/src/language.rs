
pub enum Language {
  Javascript,
  Typescript,
}

impl Language {
  pub fn from(language: &str) -> Self {
      match language {
          "javascript" | "js" => Language::Javascript,
          "typescript" | "ts" => Language::Typescript,
          _ => panic!("Unknown language"),
      }
  }
}
impl ToString for Language {
  fn to_string(&self) -> String {
      match self {
          Language::Javascript => "javascript".to_string(),
          Language::Typescript => "typescript".to_string(),
      }
  }
}