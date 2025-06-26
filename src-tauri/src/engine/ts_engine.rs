use std::sync::{Arc, Mutex};

use crate::console::{add_log, clear_logs, get_logs, JavascriptValueWithType, LogConsole};
use quickjs_rs::{Context, ExecutionError, JsValue};

use swc_common::{
    comments::SingleThreadedComments,
    errors::{Emitter, DiagnosticBuilder, Handler},
    sync::Lrc,
    FileName, Globals, Mark, SourceMap, GLOBALS,
};
use swc_ecma_codegen::{to_code_default };
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsSyntax};
use swc_ecma_transforms_base::{fixer::fixer, hygiene::hygiene, resolver};
use swc_ecma_transforms_typescript::strip;


#[derive(Debug, Clone, Default)]
pub struct ErrorBuffer(Arc<Mutex<Vec<swc_common::errors::Diagnostic>>>);

impl Emitter for ErrorBuffer {
    fn emit(&mut self, db: &mut DiagnosticBuilder) {
        self.0.lock().unwrap().push((**db).clone());
    }
}
pub struct TsEngine;

impl TsEngine {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn run(&self, code: &str) -> Result<Vec<Vec<JavascriptValueWithType>>, String> {
        let js_code = self.compile_ts_with_swc(code).await?;
        self.run_js(&js_code).await
    }

    async fn run_js(&self, code: &str) -> Result<Vec<Vec<JavascriptValueWithType>>, String> {
        clear_logs();
        let context = Context::builder().console(LogConsole).build().unwrap();
        let value = match context.eval(code) {
            Ok(value) => value,
            Err(e) => {
                println!("Error executing JavaScript: {:?}", e);
                let err_msg = match e {
                    ExecutionError::Internal(e) => e,
                    ExecutionError::Conversion(e) => e.to_string(),
                    ExecutionError::Exception(e) => {
                        if let Some(s) = e.into_string() {
                            s
                        } else {
                            "Unknown error".to_string()
                        }
                    }
                    ExecutionError::InputWithZeroBytes => "Input with zero bytes".to_string(),
                    ExecutionError::OutOfMemory => "Out of memory".to_string(),
                    _ => "Unknown error".to_string(),
                };
                return Err(err_msg);
            }
        };
        match value {
            JsValue::Undefined => (),
            _ => add_log(vec![value]),
        }

        let result = get_logs();
        Ok(result)
    }

    /// Compile TypeScript code with esbuild
    // async fn compile_ts_with_esbuild(&self, ts_code: &str) -> Result<String, String> {
    //     let src = Arc::new(ts_code.as_bytes().to_vec());

    //     let mut options_builder = TransformOptionsBuilder::new();
    //     options_builder.loader = Loader::TS;
    //     let options = options_builder.build();

    //     let result = transform(src, options).await;
    //     let err = result.errors.as_slice();
    //     if !err.is_empty() {
    //         let error_messages: String = err
    //             .iter()
    //             .map(|e| e.to_string())
    //             .collect::<Vec<_>>()
    //             .join("\n");
    //         println!("TypeScript compilation errors: {:?}", error_messages);
    //         return Err(error_messages);
    //     }

    //     let js_code = result.code.to_string();
    //     Ok(js_code)
    // }

    /// Compile TypeScript code with swc
    async fn compile_ts_with_swc(&self, ts_code: &str) -> Result<String, String> {
        let cm: Lrc<SourceMap> = Default::default();
        let error_buffer = ErrorBuffer::default();
        let handler = Handler::with_emitter(false, false, Box::new(error_buffer.clone()));
        let fm = cm.new_source_file(
            Lrc::new(FileName::Custom("internal.ts".into())),
            ts_code.to_string(),
        );
        let comments = SingleThreadedComments::default();
        let lexer = Lexer::new(
            Syntax::Typescript(TsSyntax {
                tsx: true,
                ..Default::default()
            }),
            Default::default(),
            StringInput::from(&*fm),
            Some(&comments),
        );
        let mut parser = Parser::new_from(lexer);

        let module = match parser.parse_program() {
            Ok(module) => module,
            Err(err) => {
                err.clone().into_diagnostic(&handler).emit();

                // return Err(format!("{:?}", err));
                // get error from error_buffer
                let errors = error_buffer.0.lock().unwrap().clone();
                return Err(format!(
                    "{}",
                    errors
                        .iter()
                        .map(|diagnostic| {
                            println!("{:?}", diagnostic);
                            diagnostic.message()
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                ));
            }
        };
        
        let globals = Globals::default();
        GLOBALS.set(&globals, || {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            // Optionally transforms decorators here before the resolver pass
            // as it might produce runtime declarations.

            // Conduct identifier scope analysis
            let module = module.apply(resolver(unresolved_mark, top_level_mark, true));

            // Remove typescript types
            let module = module.apply(strip(unresolved_mark, top_level_mark));

            // Fix up any identifiers with the same name, but different contexts
            let module = module.apply(hygiene());

            // Ensure that we have enough parenthesis.
            let program = module.apply(fixer(Some(&comments)));

            let code = to_code_default(cm, Some(&comments), &program);
            return Ok(code);
        })
    }
}
