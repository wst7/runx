use crate::{console::JavascriptValueWithType, engine::ts_engine::TsEngine, language::Language};

mod ts_engine;
pub struct Engine {
    language: Language,
}

impl Engine {
    pub fn new(language: Language) -> Self {
        Self { language }
    }
    /// return JavascriptValueWithType for frontend processing
    pub async fn run(&mut self, code: &str) -> Result<Vec<Vec<JavascriptValueWithType>>, String> {
        match self.language {
            Language::Typescript => TsEngine::new().run(code).await,
            _ => Err("Unknown language".to_string()),
        }
    }
}
