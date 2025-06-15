pub mod run;

#[cfg(feature = "lisp")]
mod lisp;

#[cfg(feature = "python_bindings")]
mod python_bindings;
#[cfg(feature = "python_bindings")]
pub use python_bindings::spnl;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Document {
    Text(String),
    Binary(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Unit {
    /// User prompt
    User((String,)),

    /// System prompt
    System((String,)),

    /// Print a helpful message to the console
    Print((String,)),

    /// Reduce
    Cross(Vec<Unit>),

    /// Map
    Plus(Vec<Unit>),

    /// Helpful for repeating an operation n times in a Plus
    Repeat((usize, Box<Unit>)),

    /// (model, input, max_tokens, temperature, accumulate?)
    #[serde(rename = "g")]
    Generate((String, Box<Unit>, i32, f32, bool)),

    /// Ask with a given message
    Ask((String,)),

    /// (embedding_model, question, docs): Incorporate information relevant to the
    /// question gathered from the given docs
    #[cfg(feature = "rag")]
    Retrieve((String, Box<Unit>, (String, Document))),
}

#[cfg(feature = "cli_support")]
fn truncate(s: &str, max_chars: usize) -> String {
    if s.len() < max_chars {
        return s.to_string();
    }

    match s.char_indices().nth(max_chars) {
        None => s.to_string(),
        Some((idx, _)) => format!("{}…", &s[..idx]),
    }
}

#[cfg(feature = "cli_support")]
impl ptree::TreeItem for Unit {
    type Child = Self;
    fn write_self<W: ::std::io::Write>(
        &self,
        f: &mut W,
        style: &ptree::Style,
    ) -> ::std::io::Result<()> {
        write!(
            f,
            "{}",
            match self {
                Unit::User((s,)) =>
                    style.paint(format!("\x1b[33mUser\x1b[0m {}", truncate(s, 700))),
                Unit::System((s,)) =>
                    style.paint(format!("\x1b[34mSystem\x1b[0m {}", truncate(s, 700))),
                Unit::Plus(_) => style.paint("\x1b[31;1mPlus\x1b[0m".to_string()),
                Unit::Cross(_) => style.paint("\x1b[31;1mCross\x1b[0m".to_string()),
                Unit::Generate((m, _, _, _, accumulate)) => style.paint(format!(
                    "\x1b[31;1mGenerate\x1b[0m \x1b[2m{m}\x1b[0m accumulate?={accumulate}"
                )),
                Unit::Repeat((n, _)) => style.paint(format!("Repeat {n}")),
                Unit::Ask((m,)) => style.paint(format!("Ask {m}")),
                Unit::Print((m,)) => style.paint(format!("Print {}", truncate(m, 700))),
                #[cfg(feature = "rag")]
                Unit::Retrieve((_, _, _)) => style.paint("\x1b[34;1mAugment\x1b[0m".to_string()),
            }
        )
    }
    fn children(&self) -> ::std::borrow::Cow<[Self::Child]> {
        ::std::borrow::Cow::from(match self {
            Unit::Ask(_) | Unit::User(_) | Unit::System(_) | Unit::Print(_) => vec![],
            Unit::Plus(v) | Unit::Cross(v) => v.clone(),
            Unit::Repeat((_, v)) => vec![*v.clone()],
            Unit::Generate((_, i, _, _, _)) => vec![*i.clone()],
            #[cfg(feature = "rag")]
            Unit::Retrieve((_, body, (filename, _))) => vec![
                *body.clone(),
                Unit::User((format!("<augmentation document: {filename}>"),)),
            ],
        })
    }
}
impl ::std::fmt::Display for Unit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Unit::Cross(v) | Unit::Plus(v) => write!(
                f,
                "{}",
                v.iter()
                    .map(|u| format!("{}", u))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            Unit::System((s,)) | Unit::User((s,)) => write!(f, "{}", s),
            _ => Ok(()),
        }
    }
}
impl From<&str> for Unit {
    fn from(s: &str) -> Self {
        Self::User((s.into(),))
    }
}
impl ::std::str::FromStr for Unit {
    type Err = Box<dyn ::std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::User((s.to_string(),)))
    }
}
impl From<&String> for Unit {
    fn from(s: &String) -> Self {
        Self::User((s.clone(),))
    }
}

/// Pretty print a query
pub fn pretty_print(u: &Unit) -> serde_lexpr::Result<()> {
    println!("{}", serde_lexpr::to_string(u)?);
    Ok(())
}

/// Deserialize a SPNL query from a string
pub fn from_str(s: &str) -> serde_lexpr::Result<Unit> {
    Ok(serde_lexpr::from_str(s)?)
}

/// Deserialize a SPNL query from a reader
pub fn from_reader(r: impl ::std::io::Read) -> serde_lexpr::error::Result<Unit> {
    serde_lexpr::from_reader(r)
}

/// Deserialize a SPNL query from a file path
pub fn from_file(f: &str) -> serde_lexpr::error::Result<Unit> {
    serde_lexpr::from_reader(::std::fs::File::open(f)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_user() -> serde_lexpr::Result<()> {
        let result = from_str("(user \"hello\")")?;
        assert_eq!(result, Unit::User(("hello".to_string(),)));
        Ok(())
    }

    #[test]
    fn serde_system() -> serde_lexpr::Result<()> {
        let result = from_str("(system \"hello\")")?;
        assert_eq!(result, Unit::System(("hello".to_string(),)));
        Ok(())
    }

    #[test]
    fn serde_ask() -> serde_lexpr::Result<()> {
        let result = from_str("(ask \"hello\")")?;
        assert_eq!(result, Unit::Ask(("hello".to_string(),)));
        Ok(())
    }

    #[test]
    fn serde_plus_1() -> serde_lexpr::Result<()> {
        let result = from_str("(plus (user \"hello\"))")?;
        assert_eq!(result, Unit::Plus(vec![Unit::User(("hello".to_string(),))]));
        Ok(())
    }

    #[test]
    fn serde_plus_2() -> serde_lexpr::Result<()> {
        let result = from_str("(plus (user \"hello\") (system \"world\"))")?;
        assert_eq!(
            result,
            Unit::Plus(vec![
                Unit::User(("hello".to_string(),)),
                Unit::System(("world".to_string(),))
            ])
        );
        Ok(())
    }

    #[test]
    fn serde_cross_1() -> serde_lexpr::Result<()> {
        let result = from_str("(cross (user \"hello\"))")?;
        assert_eq!(
            result,
            Unit::Cross(vec![Unit::User(("hello".to_string(),))])
        );
        Ok(())
    }

    #[test]
    fn serde_cross_3() -> serde_lexpr::Result<()> {
        let result =
            from_str("(cross (user \"hello\") (system \"world\") (plus (user \"sloop\")))")?;
        assert_eq!(
            result,
            Unit::Cross(vec![
                Unit::User(("hello".to_string(),)),
                Unit::System(("world".to_string(),)),
                Unit::Plus(vec![Unit::User(("sloop".to_string(),))])
            ])
        );
        Ok(())
    }

    #[test]
    fn serde_gen() -> serde_lexpr::Result<()> {
        let result = from_str("(g \"ollama/granite3.2:2b\" (user \"hello\") 0 0.0 #f)")?;
        assert_eq!(
            result,
            Unit::Generate((
                "ollama/granite3.2:2b".to_string(),
                Box::new(Unit::User(("hello".to_string(),))),
                0,
                0.0,
                false
            ))
        );
        Ok(())
    }
}
