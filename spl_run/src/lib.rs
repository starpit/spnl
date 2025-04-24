use futures::future::{BoxFuture, FutureExt};

use crate::plan::plan;
use crate::result::{SplEval, SplResult};
use spl_ast::Unit;

pub mod generate;
pub mod plan;
pub mod result;

fn fold<'a>(_v: &Vec<Unit<'a>>) -> SplResult<'a> {
    //Ok(v.iter().fold(run).collec
    Ok(SplEval::Bool(true))
}

async fn map<'a>(units: Vec<Unit<'a>>) -> SplResult<'a> {
    Ok(SplEval::List(futures::future::try_join_all(units.iter().map(move |u| run(u))).await?))
}

pub async fn run<'a>(unit: &'a Unit<'a>) -> SplResult<'a> {
    match plan(unit) {
        Unit::Number(n) => Ok(SplEval::Number(n.clone())),
        Unit::Bool(b) => Ok(SplEval::Bool(b.clone())),
        Unit::Slice(s) => Ok(SplEval::Slice(s)),
        Unit::String(s) => Ok(SplEval::String(s)),
        Unit::Cross(c) => fold(&c),
        Unit::Plus(c) => map(c.clone()).await,
        Unit::Generate((model, input)) => generate::generate(model, &input).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), SplError> {
        let result = run(&Unit::Bool(true))?;
        assert_eq!(result, SplEval::Bool(true));
        Ok(())
    }
}
