use crate::Query;
use crate::run::result::SpnlError;

pub enum EmbedData {
    Query(Query),
    Vec(Vec<String>),
}

pub async fn embed(embedding_model: &String, data: EmbedData) -> Result<Vec<Vec<f32>>, SpnlError> {
    match embedding_model {
        #[cfg(feature = "ollama")]
        m if m.starts_with("ollama/") => crate::run::backend::ollama::embed(&m[7..], &data).await,

        #[cfg(feature = "ollama")]
        m if m.starts_with("ollama_chat/") => {
            crate::run::backend::ollama::embed(&m[12..], &data).await
        }

        #[cfg(feature = "openai")]
        m if m.starts_with("openai/") => {
            todo!()
            //crate::run::openai::generate_openai(&m[7..], input, max_tokens, temp, mp).await
        }

        _ => todo!(),
    }
}
