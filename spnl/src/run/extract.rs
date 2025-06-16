use crate::{Generate, Query};

/// Extract models referenced by the program
pub fn extract_models(program: &Query) -> Vec<String> {
    extract_values(program, "model")
}

/// Take a list of Yaml fragments and produce a vector of the string-valued entries of the given field
fn extract_values(program: &Query, field: &str) -> Vec<String> {
    let mut values = vec![];
    extract_values_iter(program, field, &mut values);

    // A single program may specify the same model more than once. Dedup!
    values.sort();
    values.dedup();

    values
}

/// Produce a vector of the string-valued entries of the given field
fn extract_values_iter(program: &Query, field: &str, values: &mut Vec<String>) {
    match program {
        #[cfg(feature = "rag")]
        Query::Retrieve(crate::Retrieve {
            embedding_model, ..
        }) => values.push(embedding_model.clone()),
        Query::Generate(Generate { model, .. }) => values.push(model.clone()),
        Query::Plus(v) | Query::Cross(v) => {
            v.iter()
                .for_each(|vv| extract_values_iter(vv, field, values));
        }
        _ => {}
    }
}
