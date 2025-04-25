use async_recursion::async_recursion;
use tokio::io::{AsyncWriteExt, stdout};

use crate::plan::plan;
use crate::result::SplResult;
use spl_ast::Unit;

pub mod generate;
pub mod plan;
pub mod result;

async fn fold(description: String, units: &Vec<Unit>) -> SplResult {
    let evaluated = futures::future::try_join_all(units.iter().map(|u| run(u, true))).await?;
    let mut stdout = stdout();
    stdout.write_all(description.as_bytes()).await?;
    stdout.write_all(b"\n").await?;
    stdout.flush().await?;
    Ok(Unit::Plus((description, evaluated)))
}

async fn map(description: String, units: &Vec<Unit>) -> SplResult {
    let mut stdout = stdout();
    stdout.write_all(description.as_bytes()).await?;
    stdout.write_all(b"\n").await?;
    stdout.flush().await?;
    let evaluated = futures::future::try_join_all(units.iter().map(|u| run(u, true))).await?;
    Ok(Unit::Plus((description, evaluated)))
}

#[async_recursion]
pub async fn run(unit: &Unit, quiet: bool) -> SplResult {
    let p = plan(unit);
    match p {
        Unit::Number(n) => Ok(Unit::Number(n.clone())),
        Unit::Bool(b) => Ok(Unit::Bool(b.clone())),
        Unit::String(s) => Ok(Unit::String(s.clone())),
        Unit::Cross((d, u)) => fold(d, &u).await,
        Unit::Plus((d, u)) => map(d, &u).await,
        Unit::Generate((model, input)) => {
            generate::generate(model.as_str(), &run(&input, quiet).await?, quiet).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), SplError> {
        let result = run(&Unit::Bool(true), false)?;
        assert_eq!(result, Unit::Bool(true));
        Ok(())
    }
}
