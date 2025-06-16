pub type SpnlError = Box<dyn ::std::error::Error + Send + Sync>;
pub type SpnlResult = Result<crate::Query, SpnlError>;
