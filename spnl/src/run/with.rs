use futures::future::try_join_all;
use itertools::Itertools;

use crate::{
    Unit,
    run::{embed::EmbedData, result::SpnlResult},
};

mod storage;

pub async fn embed_and_retrieve(
    embedding_model: &String,
    body: &Unit,
    docs: &Vec<(String, String)>,
) -> SpnlResult {
    let db_uri = "data/spnl"; // TODO
    let table_name = "spnl_vectors"; // TODO
    let db_async = storage::VecDB::connect(db_uri, table_name);

    // TODO create-if-needed
    let docs_filenames = docs
        .iter()
        .map(|(filename, _)| filename.as_str())
        .collect::<Vec<_>>();
    let docs_vectors = crate::run::generate::embed(
        embedding_model,
        &EmbedData::Vec(
            docs.into_iter()
                .map(|(_, content)| content.clone())
                .collect(),
        ),
    )
    .await?;
    let db = db_async.await?;
    db.add_vector(docs_filenames.as_slice(), docs_vectors, 1024)
        .await?;

    let body_vectors =
        crate::run::generate::embed(embedding_model, &EmbedData::Unit(body.clone())).await?;

    let matching_docs = try_join_all(body_vectors.into_iter().map(|v| db.find_similar(v, 1)))
        .await?
        .into_iter()
        .flatten()
        .filter_map(|record_batch| {
            if let Some(files_array) = record_batch.column_by_name("filename") {
                if let Some(files) = files_array
                    .as_any()
                    .downcast_ref::<arrow_array::StringArray>()
                {
                    return Some(
                        files
                            .iter()
                            .filter_map(|b| b.map(|b| b.to_string()))
                            .collect::<Vec<String>>(),
                    );
                }
            }

            // no matching docs for this body vector
            None
        })
        .flatten()
        .unique()
        .collect::<Vec<_>>();

    let matching_content = docs
        .iter()
        .filter_map(
            |(filename, content)| match matching_docs.iter().any(|f| f == filename) {
                true => Some(Unit::User((content.clone(),))),
                false => None,
            },
        )
        .collect::<Vec<_>>();

    // no matching documents
    Ok(Unit::Plus(matching_content))
}
