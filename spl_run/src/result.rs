use spl_ast::Unit;

pub type SplError = Box<dyn ::std::error::Error + Send + Sync>;
pub type SplResult = Result<Unit, SplError>;
