use indicatif::MultiProgress;

use crate::Unit;
use crate::run::result::{SpnlError, SpnlResult};

pub async fn generate(
    model: &str,
    input: &Unit,
    max_tokens: i32,
    temp: f32,
    mp: Option<&MultiProgress>,
) -> SpnlResult {
    match model {
        #[cfg(feature = "ollama")]
        m if m.starts_with("ollama/") => {
            crate::run::ollama::generate_ollama(&m[7..], input, max_tokens, temp, mp).await
        }

        #[cfg(feature = "ollama")]
        m if m.starts_with("ollama_chat/") => {
            crate::run::ollama::generate_ollama(&m[12..], input, max_tokens, temp, mp).await
        }

        #[cfg(feature = "openai")]
        m if m.starts_with("openai/") => {
            crate::run::openai::generate_openai(&m[7..], input, max_tokens, temp, mp).await
        }

        _ => todo!(),
    }
}

pub async fn embed(
    embedding_model: &String,
    data: &crate::run::embed::EmbedData,
) -> Result<Vec<Vec<f32>>, SpnlError> {
    match embedding_model {
        #[cfg(feature = "ollama")]
        m if m.starts_with("ollama/") => crate::run::ollama::embed(&m[7..], data).await,

        #[cfg(feature = "ollama")]
        m if m.starts_with("ollama_chat/") => crate::run::ollama::embed(&m[12..], data).await,

        #[cfg(feature = "openai")]
        m if m.starts_with("openai/") => {
            todo!()
            //crate::run::openai::generate_openai(&m[7..], input, max_tokens, temp, mp).await
        }

        _ => todo!(),
    }
}
