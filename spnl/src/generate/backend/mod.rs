#[cfg(feature = "openai")]
pub(crate) mod openai;

#[cfg(feature = "spnl-api")]
pub(crate) mod spnl;

#[cfg(feature = "candle")]
pub(crate) mod candle;

pub(crate) mod capabilities;

mod progress;
