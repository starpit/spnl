use futures::future::try_join_all;
use itertools::Itertools;

use crate::{
    Document, Unit,
    run::{
        embed::EmbedData,
        result::{SpnlError, SpnlResult},
    },
};

mod storage;

/// e.g. bytes="a\nb\nc\nd", window_width=2 -> ["a\nb", "b\nc", "c\nd"]
fn windowed_pdf(bytes: &Vec<u8>, window_width: usize) -> Result<Vec<String>, SpnlError> {
    Ok(pdf_extract::extract_text_from_mem(&bytes)?
        .lines()
        .filter(|s| s.len() > 0)
        .collect::<Vec<_>>()
        .windows(window_width)
        .map(|s| s.join("\n"))
        .collect())
}

pub async fn embed_and_retrieve(
    embedding_model: &String,
    body: &Unit,
    docs: &Vec<(String, Document)>,
    db_uri: &str,
    table_name: &str,
) -> SpnlResult {
    let max_matches = 100; // TODO allow to be specified in query
    let db_async = storage::VecDB::connect(db_uri, table_name);

    let docs_content = docs
        .into_iter()
        .map(|(filename, content)| {
            match (
                content,
                ::std::path::Path::new(filename)
                    .extension()
                    .and_then(std::ffi::OsStr::to_str),
            ) {
                (Document::Text(content), _) => Ok(vec![content.clone()]),
                (Document::Binary(content), Some("pdf")) => windowed_pdf(&content, 4),
                _ => Err(Box::from(format!(
                    "Unsupported `with` binary document {filename}"
                ))),
            }
        })
        .collect::<Result<Vec<Vec<String>>, SpnlError>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<String>>();
    let docs_vectors =
        crate::run::generate::embed(embedding_model, &EmbedData::Vec(docs_content.clone())).await?;
    let db = db_async.await?;
    // TODO create-if-needed
    db.add_vector(docs_content.as_slice(), docs_vectors, 1024)
        .await?;

    let body_vectors =
        crate::run::generate::embed(embedding_model, &EmbedData::Unit(body.clone())).await?;

    let matching_docs = try_join_all(
        body_vectors
            .into_iter()
            .map(|v| db.find_similar(v, max_matches)),
    )
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
    .map(|doc| Unit::User((doc,)))
    .collect::<Vec<_>>();

    Ok(Unit::Plus(matching_docs))
}
