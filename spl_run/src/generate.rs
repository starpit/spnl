use indicatif::MultiProgress;

use crate::result::SplResult;
use spl_ast::Unit;

use crate::ollama::generate_ollama;
use crate::openai::generate_openai;

pub async fn generate(
    model: &str,
    input: &Unit,
    max_tokens: i32,
    temp: f32,
    m: Option<&MultiProgress>,
) -> SplResult {
    if model.starts_with("ollama/") || model.starts_with("ollama_chat/") {
        let model = if model.starts_with("ollama/") {
            &model[7..]
        } else {
            &model[12..]
        };

        generate_ollama(model, input, max_tokens, temp, m).await
    } else if model.starts_with("openai/") {
        generate_openai(&model[7..], input, max_tokens, temp, m).await
    } else {
        todo!()
    }
}
