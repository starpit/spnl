#[cfg(feature = "python_bindings")]
use pyo3::prelude::*;

#[pymodule]
#[cfg(feature = "python_bindings")]
pub fn spnl(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[cfg(feature = "tok")]
    m.add_class::<crate::run::tokenizer::TokenizedQuery>()?;

    #[cfg(feature = "tok")]
    m.add_function(wrap_pyfunction!(crate::run::tokenizer::tokenize_query, m)?)?;

    #[cfg(feature = "tok")]
    m.add_function(wrap_pyfunction!(crate::run::tokenizer::tokenize_plus, m)?)?;

    Ok(())
}
