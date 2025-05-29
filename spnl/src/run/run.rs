use async_recursion::async_recursion;
use indicatif::MultiProgress;

use crate::{Unit, run::result::SpnlResult};

async fn cross(units: &Vec<Unit>, mm: Option<&MultiProgress>) -> SpnlResult {
    let mym = MultiProgress::new();
    let m = if let Some(m) = mm { m } else { &mym };

    let mut iter = units.iter();
    let mut evaluated = vec![];
    while let Some(u) = iter.next() {
        evaluated.push(run(u, Some(m)).await?);
    }

    Ok(Unit::Cross(evaluated))
}

async fn plus(units: &Vec<Unit>) -> SpnlResult {
    let m = MultiProgress::new();
    let evaluated = futures::future::try_join_all(units.iter().map(|u| run(u, Some(&m)))).await?;

    if evaluated.len() == 1 {
        Ok(evaluated[0].clone())
    } else {
        Ok(Unit::Plus(evaluated))
    }
}

#[async_recursion]
pub async fn run(unit: &Unit, m: Option<&MultiProgress>) -> SpnlResult {
    #[cfg(feature = "pull")]
    let _ = crate::run::pull::pull_if_needed(unit).await?;

    match unit {
        Unit::Print((m,)) => {
            println!("{}", m);
            Ok(Unit::Print((m.clone(),)))
        }
        Unit::User(s) => Ok(Unit::User(s.clone())),
        Unit::System(s) => Ok(Unit::System(s.clone())),

        #[cfg(feature = "rag")]
        Unit::Retrieve((embedding_model, body, docs)) => {
            crate::run::with::embed_and_retrieve(embedding_model, body, docs).await
        }
        #[cfg(not(feature = "rag"))]
        Unit::Retrieve((embedding_model, body, docs)) => Err(Box::from("rag feature not enabled")),

        Unit::Cross(u) => cross(&u, m).await,
        Unit::Plus(u) => plus(&u).await,
        Unit::Generate((model, input, max_tokens, temp)) => {
            crate::run::generate::generate(
                model.as_str(),
                &run(&input, m).await?,
                *max_tokens,
                *temp,
                m,
            )
            .await
        }

        #[cfg(not(feature = "cli_support"))]
        Unit::Ask((message,)) => todo!(),
        #[cfg(feature = "cli_support")]
        Unit::Ask((message,)) => {
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
            Ok(Unit::User((prompt,)))
        }

        // should not happen
        Unit::Repeat(_) => todo!(),

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
    use crate::run::result::SpnlError;

    #[tokio::test]
    async fn it_works() -> Result<(), SpnlError> {
        let result = run(&"hello".into(), None).await?;
        assert_eq!(result, Unit::User(("hello".to_string(),)));
        Ok(())
    }
}
