#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Document {
    Text(String),
    Binary(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Query {
    /// User prompt
    User((String,)),

    /// System prompt
    System((String,)),

    /// Print a helpful message to the console
    Print((String,)),

    /// Reduce
    Cross(Vec<Query>),

    /// Map
    Plus(Vec<Query>),

    /// Helpful for repeating an operation n times in a Plus
    Repeat((usize, Box<Query>)),

    /// (model, input, max_tokens, temperature, accumulate?)
    #[serde(rename = "g")]
    Generate((String, Box<Query>, i32, f32, bool)),

    /// Ask with a given message
    Ask((String,)),

    /// (embedding_model, question, docs): Incorporate information relevant to the
    /// question gathered from the given docs
    #[cfg(feature = "rag")]
    Retrieve((String, Box<Query>, (String, Document))),
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
impl ptree::TreeItem for Query {
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
                Query::User((s,)) =>
                    style.paint(format!("\x1b[33mUser\x1b[0m {}", truncate(s, 700))),
                Query::System((s,)) =>
                    style.paint(format!("\x1b[34mSystem\x1b[0m {}", truncate(s, 700))),
                Query::Plus(_) => style.paint("\x1b[31;1mPlus\x1b[0m".to_string()),
                Query::Cross(_) => style.paint("\x1b[31;1mCross\x1b[0m".to_string()),
                Query::Generate((m, _, _, _, accumulate)) => style.paint(format!(
                    "\x1b[31;1mGenerate\x1b[0m \x1b[2m{m}\x1b[0m accumulate?={accumulate}"
                )),
                Query::Repeat((n, _)) => style.paint(format!("Repeat {n}")),
                Query::Ask((m,)) => style.paint(format!("Ask {m}")),
                Query::Print((m,)) => style.paint(format!("Print {}", truncate(m, 700))),
                #[cfg(feature = "rag")]
                Query::Retrieve((_, _, _)) => style.paint("\x1b[34;1mAugment\x1b[0m".to_string()),
            }
        )
    }
    fn children(&self) -> ::std::borrow::Cow<[Self::Child]> {
        ::std::borrow::Cow::from(match self {
            Query::Ask(_) | Query::User(_) | Query::System(_) | Query::Print(_) => vec![],
            Query::Plus(v) | Query::Cross(v) => v.clone(),
            Query::Repeat((_, v)) => vec![*v.clone()],
            Query::Generate((_, i, _, _, _)) => vec![*i.clone()],
            #[cfg(feature = "rag")]
            Query::Retrieve((_, body, (filename, _))) => vec![
                *body.clone(),
                Query::User((format!("<augmentation document: {filename}>"),)),
            ],
        })
    }
}

impl ::std::fmt::Display for Query {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Query::Cross(v) | Query::Plus(v) => write!(
                f,
                "{}",
                v.iter()
                    .map(|u| format!("{}", u))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            Query::System((s,)) | Query::User((s,)) => write!(f, "{}", s),
            _ => Ok(()),
        }
    }
}

impl From<&str> for Query {
    fn from(s: &str) -> Self {
        Self::User((s.into(),))
    }
}

impl ::std::str::FromStr for Query {
    type Err = Box<dyn ::std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::User((s.to_string(),)))
    }
}

impl From<&String> for Query {
    fn from(s: &String) -> Self {
        Self::User((s.clone(),))
    }
}

/// Pretty print a query
pub fn pretty_print(u: &Query) -> serde_lexpr::Result<()> {
    println!("{}", serde_lexpr::to_string(u)?);
    Ok(())
}

/// Deserialize a SPNL query from a string
pub fn from_str(s: &str) -> serde_lexpr::Result<Query> {
    Ok(serde_lexpr::from_str(s)?)
}

/// Deserialize a SPNL query from a reader
pub fn from_reader(r: impl ::std::io::Read) -> serde_lexpr::error::Result<Query> {
    serde_lexpr::from_reader(r)
}

/// Deserialize a SPNL query from a file path
pub fn from_file(f: &str) -> serde_lexpr::error::Result<Query> {
    serde_lexpr::from_reader(::std::fs::File::open(f)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_user() -> serde_lexpr::Result<()> {
        let result = from_str("(user \"hello\")")?;
        assert_eq!(result, Query::User(("hello".to_string(),)));
        Ok(())
    }

    #[test]
    fn serde_system() -> serde_lexpr::Result<()> {
        let result = from_str("(system \"hello\")")?;
        assert_eq!(result, Query::System(("hello".to_string(),)));
        Ok(())
    }

    #[test]
    fn serde_ask() -> serde_lexpr::Result<()> {
        let result = from_str("(ask \"hello\")")?;
        assert_eq!(result, Query::Ask(("hello".to_string(),)));
        Ok(())
    }

    #[test]
    fn serde_plus_1() -> serde_lexpr::Result<()> {
        let result = from_str("(plus (user \"hello\"))")?;
        assert_eq!(
            result,
            Query::Plus(vec![Query::User(("hello".to_string(),))])
        );
        Ok(())
    }

    #[test]
    fn serde_plus_2() -> serde_lexpr::Result<()> {
        let result = from_str("(plus (user \"hello\") (system \"world\"))")?;
        assert_eq!(
            result,
            Query::Plus(vec![
                Query::User(("hello".to_string(),)),
                Query::System(("world".to_string(),))
            ])
        );
        Ok(())
    }

    #[test]
    fn serde_cross_1() -> serde_lexpr::Result<()> {
        let result = from_str("(cross (user \"hello\"))")?;
        assert_eq!(
            result,
            Query::Cross(vec![Query::User(("hello".to_string(),))])
        );
        Ok(())
    }

    #[test]
    fn serde_cross_3() -> serde_lexpr::Result<()> {
        let result =
            from_str("(cross (user \"hello\") (system \"world\") (plus (user \"sloop\")))")?;
        assert_eq!(
            result,
            Query::Cross(vec![
                Query::User(("hello".to_string(),)),
                Query::System(("world".to_string(),)),
                Query::Plus(vec![Query::User(("sloop".to_string(),))])
            ])
        );
        Ok(())
    }

    #[test]
    fn serde_gen() -> serde_lexpr::Result<()> {
        let result = from_str("(g \"ollama/granite3.2:2b\" (user \"hello\") 0 0.0 #f)")?;
        assert_eq!(
            result,
            Query::Generate((
                "ollama/granite3.2:2b".to_string(),
                Box::new(Query::User(("hello".to_string(),))),
                0,
                0.0,
                false
            ))
        );
        Ok(())
    }
}
