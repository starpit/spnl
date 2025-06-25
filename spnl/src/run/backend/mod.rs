#[cfg(feature = "ollama")]
pub(crate) mod ollama;

#[cfg(feature = "openai")]
pub(crate) mod openai;

#[cfg(feature = "spnl-api")]
pub(crate) mod spnl;
