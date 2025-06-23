use pyo3::prelude::*;

#[pymodule]
pub fn spnl(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[cfg(feature = "tok")]
    m.add_class::<crate::run::tokenizer::TokenizedQuery>()?;

    #[cfg(feature = "tok")]
    m.add_function(wrap_pyfunction!(crate::run::tokenizer::tokenize_query, m)?)?;

    #[cfg(feature = "tok")]
    m.add_function(wrap_pyfunction!(crate::run::tokenizer::tokenize_plus, m)?)?;

    #[cfg(feature = "tok")]
    m.add_function(wrap_pyfunction!(crate::run::tokenizer::init, m)?)?;

    //m.add_class::<SimpleQuery>()?;
    //m.add_class::<SimpleGenerate>()?;
    //m.add_class::<SimpleGenerateInput>()?;
    //m.add_class::<SimpleMessage>()?;

    Ok(())
}

//#[pyclass]
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NonGenerateInput {
    User(String),
    System(String),
    Plus(Vec<NonGenerateInput>),
    Cross(Vec<NonGenerateInput>),
}

//#[pyclass]
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SingleGenerate {
    pub model: String,
    pub input: NonGenerateInput,
    pub max_tokens: Option<i32>,
    pub temperature: Option<f32>,
}

//#[pyclass]
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SimpleQuery {
    pub g: SingleGenerate,
}

impl From<NonGenerateInput> for crate::Query {
    fn from(input: NonGenerateInput) -> Self {
        match input {
            NonGenerateInput::User(m) => crate::Query::User(m.clone()),
            NonGenerateInput::System(m) => crate::Query::System(m.clone()),
            NonGenerateInput::Plus(v) => {
                crate::Query::Plus(v.into_iter().map(|m| m.into()).collect())
            }
            NonGenerateInput::Cross(v) => {
                crate::Query::Cross(v.into_iter().map(|m| m.into()).collect())
            }
        }
    }
}

impl From<SimpleQuery> for crate::Query {
    fn from(q: SimpleQuery) -> Self {
        Self::Generate(crate::Generate {
            model: q.g.model.clone(),
            input: Box::new(q.g.input.clone().into()),
            max_tokens: q.g.max_tokens,
            temperature: q.g.temperature,
            accumulate: None,
        })
    }
}
