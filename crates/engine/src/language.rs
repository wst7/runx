#[derive(Debug, Clone, Copy)]
pub enum Language {
    Javascript,
    Python,
}

impl Language {
    pub fn from(language: &str) -> Self {
        match language {
            "python" | "py" => Language::Python,
            "javascript" | "typescript" | "js" | "ts" => Language::Javascript,
            _ => panic!("Unknown language"),
        }
    }
}
