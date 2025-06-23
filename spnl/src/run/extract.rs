use crate::{Generate, Query};

/// Extract models referenced by the query
pub fn extract_models(query: &Query) -> Vec<String> {
    let mut values = vec![];
    extract_values_iter(query, &mut values);

    // A single query may specify the same model more than once. Dedup!
    values.sort();
    values.dedup();

    values
}

/// Produce a vector of the models used by the given `query`
fn extract_values_iter(query: &Query, values: &mut Vec<String>) {
    match query {
        #[cfg(feature = "rag")]
        Query::Retrieve(crate::Retrieve {
            embedding_model, ..
        }) => values.push(embedding_model.clone()),
        Query::Generate(Generate { model, .. }) => values.push(model.clone()),
        Query::Plus(v) | Query::Cross(v) => {
            v.iter().for_each(|vv| extract_values_iter(vv, values));
        }
        _ => {}
    }
}
