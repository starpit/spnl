pub type SplError = Box<dyn ::std::error::Error + Send + Sync>;
pub type SplResult<'a> = Result<SplEval<'a>, SplError>;

#[derive(Debug)]
pub enum SplEval<'a> {
    Bool(bool),
    Number(usize),
    Slice(&'a str),
    String(String),
    List(Vec<SplEval<'a>>),
}

impl<'a> PartialEq for SplEval<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SplEval::Bool(a), SplEval::Bool(b)) => a == b,
            (SplEval::Number(a), SplEval::Number(b)) => a == b,
            (SplEval::Slice(a), SplEval::Slice(b)) => a == b,
            (SplEval::String(a), SplEval::String(b)) => a == b,
            (SplEval::List(a), SplEval::List(b)) => a == b,
            _ => false,
        }
    }
}
