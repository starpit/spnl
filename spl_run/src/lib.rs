use async_recursion::async_recursion;
use indicatif::MultiProgress;

use crate::plan::plan;
use crate::result::SplResult;
use spl_ast::Unit;

pub mod generate;
pub mod plan;
pub mod result;

async fn fold(description: String, units: &Vec<Unit>) -> SplResult {
    let m = MultiProgress::new();
    let evaluated = futures::future::try_join_all(units.iter().map(|u| run(u, Some(&m)))).await?;
    m.println(format!("\x1b[1mCross: \x1b[0m{}", &description))?;
    Ok(Unit::Plus((description, evaluated)))
}

async fn map(description: String, units: &Vec<Unit>) -> SplResult {
    let m = MultiProgress::new();
    m.println(format!("\x1b[1mPlus: \x1b[0m{}", &description))?;
    let evaluated = futures::future::try_join_all(units.iter().map(|u| run(u, Some(&m)))).await?;
    Ok(Unit::Plus((description, evaluated)))
}

#[async_recursion]
pub async fn run(unit: &Unit, m: Option<&MultiProgress>) -> SplResult {
    let p = plan(unit);
    match p {
        Unit::String(s) => Ok(Unit::String(s.clone())),
        Unit::Cross((d, u)) => fold(d, &u).await,
        Unit::Plus((d, u)) => map(d, &u).await,
        Unit::Generate((model, input, max_tokens, temp)) => {
            generate::generate(model.as_str(), &run(&input, m).await?, max_tokens, temp, m).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::SplError;

    #[tokio::test]
    async fn it_works() -> Result<(), SplError> {
        let result = run(&"hello".into(), None).await?;
        assert_eq!(result, Unit::String("hello".to_string()));
        Ok(())
    }
}
