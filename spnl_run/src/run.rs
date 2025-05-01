use async_recursion::async_recursion;
use indicatif::MultiProgress;

use crate::generate::generate;
use crate::plan::plan;
use crate::pull::pull_if_needed;
use crate::result::SpnlResult;
use spnl_ast::Unit;

async fn cross(
    description: Option<String>,
    units: &Vec<Unit>,
    mm: Option<&MultiProgress>,
) -> SpnlResult {
    let mym = MultiProgress::new();
    let m = if let Some(m) = mm { m } else { &mym };
    let evaluated = futures::future::try_join_all(units.iter().map(|u| run(u, Some(m)))).await?;
    if let Some(description) = &description {
        m.println(format!("\x1b[1mCross: \x1b[0m{}", description))?;
    }
    Ok(Unit::Plus((description, evaluated)))
}

async fn plus(description: Option<String>, units: &Vec<Unit>) -> SpnlResult {
    let m = MultiProgress::new();
    if let Some(description) = &description {
        m.println(format!("\x1b[1mPlus: \x1b[0m{}", description))?;
    }
    let evaluated = futures::future::try_join_all(units.iter().map(|u| run(u, Some(&m)))).await?;
    Ok(Unit::Plus((description, evaluated)))
}

#[async_recursion]
pub async fn run(unit: &Unit, m: Option<&MultiProgress>) -> SpnlResult {
    let pull_future = pull_if_needed(unit);
    let p = plan(unit);
    let _ = pull_future.await?;

    match p {
        Unit::String(s) => Ok(Unit::String(s.clone())),
        Unit::System(s) => Ok(Unit::System(s.clone())),
        Unit::Cross((d, u)) => cross(d, &u, m).await,
        Unit::Plus((d, u)) => plus(d, &u).await,
        Unit::Generate((model, input, max_tokens, temp)) => {
            generate(model.as_str(), &run(&input, m).await?, max_tokens, temp, m).await
        }

        Unit::Ask(message) => {
            use rustyline::error::ReadlineError;
            let mut rl = rustyline::DefaultEditor::new().unwrap();
            let _ = rl.load_history("history.txt");
            let prompt = match rl.readline(message.as_str()) {
                Ok(line) => {
                    rl.add_history_entry(line.as_str()).unwrap();
                    line
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    ::std::process::exit(0)
                }
                Err(err) => panic!("{}", err),
            };
            rl.append_history("history.txt").unwrap();
            Ok(Unit::String(prompt))
        }

        Unit::Loop(l) => loop {
            let mut iter = l.iter();
            while let Some(e) = iter.next() {
                run(e, m).await?;
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::SpnlError;

    #[tokio::test]
    async fn it_works() -> Result<(), SpnlError> {
        let result = run(&"hello".into(), None).await?;
        assert_eq!(result, Unit::String("hello".to_string()));
        Ok(())
    }
}
