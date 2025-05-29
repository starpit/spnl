use crate::{Unit, run::result::SpnlResult};

pub async fn embed_and_retrieve(
    embedding_model: &String,
    body: &Unit,
    docs: &Vec<String>,
) -> SpnlResult {
    //let uri = "data/sample-lancedb";
    // let db = lancedb::connect(uri).execute().await?;
    let e = crate::run::generate::embed(embedding_model, docs).await?;

    Ok("test".into())
}
