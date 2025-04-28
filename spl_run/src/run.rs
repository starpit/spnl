use async_recursion::async_recursion;
use indicatif::MultiProgress;

use crate::generate::generate;
use crate::plan::plan;
use crate::pull::pull_if_needed;
use crate::result::SplResult;
use spl_ast::Unit;

async fn cross(description: String, units: &Vec<Unit>) -> SplResult {
    let m = MultiProgress::new();
    let evaluated = futures::future::try_join_all(units.iter().map(|u| run(u, Some(&m)))).await?;
    m.println(format!("\x1b[1mCross: \x1b[0m{}", &description))?;
    Ok(Unit::Plus((description, evaluated)))
}

async fn plus(description: String, units: &Vec<Unit>) -> SplResult {
    let m = MultiProgress::new();
    m.println(format!("\x1b[1mPlus: \x1b[0m{}", &description))?;
    let evaluated = futures::future::try_join_all(units.iter().map(|u| run(u, Some(&m)))).await?;
    Ok(Unit::Plus((description, evaluated)))
}

#[async_recursion]
pub async fn run(unit: &Unit, m: Option<&MultiProgress>) -> SplResult {
    let pull_future = pull_if_needed(unit);
    let p = plan(unit);
    let _ = pull_future.await?;
    match p {
        Unit::String(s) => Ok(Unit::String(s.clone())),
        Unit::System(s) => Ok(Unit::System(s.clone())),
        Unit::Cross((d, u)) => cross(d, &u).await,
        Unit::Plus((d, u)) => plus(d, &u).await,
        Unit::Generate((model, input, max_tokens, temp)) => {
            generate(model.as_str(), &run(&input, m).await?, max_tokens, temp, m).await
        }

        Unit::Ask((message, default)) => {
            use dialoguer::Input;
            Ok(Unit::String(if let Some(default) = default {
                Input::<String>::with_theme(&MyTheme)
                    .with_prompt(message)
                    .default(default)
                    .interact_text()?
            } else {
                Input::<String>::with_theme(&MyTheme)
                    .with_prompt(message)
                    .interact_text()?
            }))
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
    use crate::result::SplError;

    #[tokio::test]
    async fn it_works() -> Result<(), SplError> {
        let result = run(&"hello".into(), None).await?;
        assert_eq!(result, Unit::String("hello".to_string()));
        Ok(())
    }
}

// avoid : suffix for Input
// https://github.com/console-rs/dialoguer/issues/255#issuecomment-1975230358
use dialoguer::theme::Theme;
use std::fmt;
pub struct MyTheme;
impl Theme for MyTheme {
    /// Formats a prompt.
    fn format_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
        write!(f, "{prompt}")
    }

    /// Formats an input prompt.
    fn format_input_prompt(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        default: Option<&str>,
    ) -> fmt::Result {
        match default {
            Some(default) => write!(f, "[{default}] {prompt}"),
            None => write!(f, "{prompt} "),
        }
    }

    /// Formats an input prompt after selection.
    fn format_input_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        selection: &str,
    ) -> fmt::Result {
        write!(f, "{prompt} {selection}")
    }
}
