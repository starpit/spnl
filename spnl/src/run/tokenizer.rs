use pyo3::prelude::*;
use tokenizers::tokenizer::Tokenizer;

use crate::{Generate, Query};

#[pyclass]
#[derive(Debug)]
pub struct TokenizedQuery {
    #[pyo3(get)]
    model: String,
    messages_: Vec<u32>,
}

#[pymethods]
impl TokenizedQuery {
    #[getter]
    fn messages(&self) -> Vec<u32> {
        return self.messages_.clone();
    }
}

fn pad(pad_token: u32, block_size: usize, toklist: Vec<u32>) -> Vec<u32> {
    toklist[0..toklist.len() - 1]
        .iter()
        .copied()
        .chain(::std::iter::repeat(pad_token).take(block_size - (toklist.len() % block_size)))
        .chain(toklist[toklist.len() - 1..].iter().copied())
        .collect()
}

fn tokenize_part(
    input: &Query,
    tok: &Tokenizer,
    pad_token: u32,
    cross_token: Option<u32>,
    plus_token: Option<u32>,
    block_size: usize,
) -> tokenizers::tokenizer::Result<Vec<u32>> {
    match input {
        Query::Cross(v) => {
            let l = v
                .iter()
                .map(|u| tokenize_part(u, tok, pad_token, cross_token, plus_token, block_size))
                .flat_map(|result| match result {
                    Ok(vec) => vec.into_iter().map(|item| Ok(item)).collect(),
                    Err(er) => vec![Err(er)],
                })
                .collect::<Result<_, _>>()?;
            if let Some(cross_token) = cross_token {
                let mut res = vec![cross_token];
                res.extend(l);
                Ok(res)
            } else {
                Ok(l)
            }
        }
        Query::Plus(v) => {
            if let Some(plus_token) = plus_token {
                v.iter()
                    .map(|u| {
                        let toks = tokenize_part(
                            u,
                            tok,
                            pad_token,
                            cross_token,
                            Some(plus_token),
                            block_size,
                        )?;
                        Ok(pad(
                            pad_token,
                            block_size,
                            [&[plus_token], &toks[..]].concat(),
                        ))
                    })
                    .flat_map(|result| match result {
                        Ok(vec) => vec.into_iter().map(|item| Ok(item)).collect(),
                        Err(er) => vec![Err(er)],
                    })
                    .collect::<Result<_, _>>()
            } else {
                v.iter()
                    .map(|u| tokenize_part(u, tok, pad_token, cross_token, plus_token, block_size))
                    .flat_map(|result| match result {
                        Ok(vec) => vec.into_iter().map(|item| Ok(item)).collect(),
                        Err(er) => vec![Err(er)],
                    })
                    .collect::<Result<_, _>>()
            }
        }
        Query::System(m) => Ok(tok
            .encode(format!("\n<|system|>\n{m}"), false)?
            .get_ids()
            .to_vec()),
        Query::User(m) => Ok(tok
            .encode(format!("\n<|user|>\n{m}"), false)?
            .get_ids()
            .to_vec()),
        _ => {
            eprintln!("Warning: Unhandled span query component {:?}", input);
            Ok(vec![])
        }
    }
}

fn handle_err(e: tokenizers::tokenizer::Error) -> PyErr {
    pyo3::exceptions::PyTypeError::new_err(format!("Error in tokenization {:?}", e))
}

fn handle_serde_err(e: serde_json::Error) -> PyErr {
    pyo3::exceptions::PyTypeError::new_err(format!("Error in deserialization {:?}", e))
}

#[pyfunction]
pub fn tokenize_query<'a>(
    query: &'a str,
    pad_token: u32,
    cross_token: Option<u32>,
    plus_token: Option<u32>,
    block_size: usize,
) -> Result<TokenizedQuery, PyErr> {
    Ok(match crate::from_str(query).map_err(handle_serde_err)? {
        Query::Generate(Generate { model, input, .. }) => {
            let tok = Tokenizer::from_pretrained(&model, None).map_err(handle_err)?;
            let messages =
                tokenize_part(&input, &tok, pad_token, cross_token, plus_token, block_size)
                    .map_err(handle_err)?
                    .into_iter()
                    .chain(
                        tok.encode("\n<|assistant|>\n", false)
                            .map_err(handle_err)?
                            .get_ids()
                            .into_iter()
                            .copied(),
                    )
                    .collect();

            TokenizedQuery {
                model: model.clone(),
                messages_: messages,
            }
        }
        _ => todo!(),
    })
}

fn extract_plus(u: &Query, in_plus: bool) -> Vec<String> {
    match (u, in_plus) {
        (Query::Cross(v), _) => v.iter().flat_map(|u| extract_plus(u, false)).collect(),
        (Query::Plus(v), _) => v.iter().flat_map(|u| extract_plus(u, true)).collect(),
        (Query::User(m), true) => vec![m.clone()],
        _ => vec![],
    }
}

#[pyfunction]
pub fn tokenize_plus<'a>(
    query: &'a str,
    pad_token: u32,
    plus_token: Option<u32>,
    block_size: usize,
) -> Result<Vec<Vec<u32>>, PyErr> {
    match crate::from_str(query).map_err(handle_serde_err)? {
        Query::Generate(Generate { model, input, .. }) => {
            let tok = Tokenizer::from_pretrained(model, None).map_err(handle_err)?;
            extract_plus(&input, false)
                .into_iter()
                .map(|s| {
                    let encoding = tok.encode(s, false)?;
                    let toks = encoding.get_ids();
                    if let Some(plus_token) = plus_token {
                        Ok(pad(
                            pad_token,
                            block_size,
                            [&[plus_token], &toks[..]].concat(),
                        ))
                    } else {
                        Ok(toks.to_vec())
                    }
                })
                .collect::<Result<_, _>>()
                .map_err(handle_err)
        }
        _ => todo!(),
    }
}
