pub mod run;

// Inspiration: https://github.com/JunSuzukiJapan/macro-lisp
#[macro_export]
macro_rules! spnl {
    // bool
    //(false) => ($crate::Unit::Bool(false));
    //(true) => ($crate::Unit::Bool(true));
    //(self $(. $e:tt)* ) => (self $(. $e)* );

    // let
    /* (let ( $( ($var:ident $e:tt) )* )
        $( ( $($e2:tt)* ) )*
    ) => ({
        $(let mut $var = $crate::spnl_arg!($e);)*
        $( $crate::spnl!( $($e2)* ) );*
    }); */

    // read as string from stdin
    (ask $message:tt) => ( $crate::Unit::Ask(($crate::spnl_arg!($message).into(),)) );

    // print a helpful message to the console
    (print $message:tt) => ( $crate::Unit::Print(($crate::spnl_arg!($message).into(),)) );

    // loop
    // (loop $( ( $($e:tt)* ) )* ) => ( loop { $( $crate::spnl!( $($e)* ) );* } );
    (loop $( ( $($e:tt)* ) )* ) => ( $crate::Unit::Loop(vec![$( $crate::spnl!( $($e)* ) ),*]) );

    // dotimes
    /*(dotimes ($var:ident $count:tt) $( ( $($e:tt)* ) )* ) => (
        for $var in 0..$crate::spnl_arg!($count) {
            $( $crate::spnl!( $($e)* ) );*
        }
    );*/

    // if
    /*(if ( $($cond:tt)* ) $e1:tt $e2:tt) => (if $crate::spnl!($($cond)*) { $crate::spnl!($e1) }else{ $crate::spnl!($e2) });
    (if ( $($cond:tt)* ) $e:tt) => (if $crate::spnl!($($cond)*) { $crate::spnl!($e) });
    (if $cond:tt $e1:tt $e2:tt) => (if $cond { $crate::spnl!($e1) }else{ $crate::spnl!($e2) });
    (if $cond:tt $e:tt) => (if $cond { $crate::spnl!($e) });*/

    // compare
    /*(eq $x:tt $y:tt) => ($crate::spnl_arg!($x) == $crate::spnl_arg!($y));
    (== $x:tt $y:tt) => ($crate::spnl_arg!($x) == $crate::spnl_arg!($y));
    (!= $x:tt $y:tt) => ($crate::spnl_arg!($x) != $crate::spnl_arg!($y));
    (< $x:tt $y:tt) => ($crate::spnl_arg!($x) < $crate::spnl_arg!($y));
    (> $x:tt $y:tt) => ($crate::spnl_arg!($x) > $crate::spnl_arg!($y));
    (<= $x:tt $y:tt) => ($crate::spnl_arg!($x) <= $crate::spnl_arg!($y));
    (>= $x:tt $y:tt) => ($crate::spnl_arg!($x) >= $crate::spnl_arg!($y));*/

    /*(print $( $e:tt )+) => ( print!( $($e),+ ) );
    (println $( $e:tt )+) => ( println!( $($e),+ ) );*/
    (format $fmt:tt $( $e:tt )*) => ( &format!($fmt, $($crate::spnl_arg!($e)),* ) );

    // math
    /*(+ $x:tt $y:tt) => ($crate::spnl_arg!($x) + $crate::spnl_arg!($y));*/
    (- $x:tt $y:tt) => ($crate::spnl_arg!($x) - $crate::spnl_arg!($y));
    /*(* $x:tt $y:tt) => ($crate::spnl_arg!($x) * $crate::spnl_arg!($y));
    (/ $x:tt $y:tt) => ($crate::spnl_arg!($x) / $crate::spnl_arg!($y));
    (% $x:tt $y:tt) => ($crate::spnl_arg!($x) % $crate::spnl_arg!($y));*/

    (file $f:tt) => (include_str!($crate::spnl_arg!($f)));
    (fetch $f:tt) => {{
        let filename = ::std::path::Path::new(file!()).parent().expect("macro to have parent directory").join($crate::spnl_arg!($f));
        ::std::fs::read_to_string(filename).expect("file to be read")
    }};

    (take $n:tt $s:tt) => (
        serde_json::from_str::<Vec<String>>($crate::spnl_arg!($s))?
            .into_iter()
            .take($crate::spnl_arg!($n).try_into().expect("usize"))
            .collect::<Vec<_>>()
    );

    (prefix $p:tt $arr:tt) => (
        $crate::spnl_arg!($arr)
            .into_iter()
            .enumerate()
            .map(|(idx, s)| ((1 + idx), s)) // (idx % $crate::spnl_arg!($chunk_size)), s))
            .map(|(idx, s)| $crate::spnl!(user (format "{}{idx}: {:?}" $p s)))
            .collect::<Vec<_>>()
    );

    (lambda ( $( $name:ident )* )
     $( ( $($e:tt)* ))*
    ) => (| $($name: Vec<Unit>),* |{ $( $crate::spnl!( $($e)* ) );* });

    (length $list:tt) => ($crate::spnl_arg!($list).len());

    (chunk $chunk_size:tt $arr:tt $f:tt) => (
        $crate::spnl_arg!($arr)
            .chunks($crate::spnl_arg!($chunk_size))
            .map(|chunk| chunk.to_vec())
            .map($crate::spnl_arg!($f))
            .collect::<Vec<_>>()
    );

    (extract $model:tt $n:tt $body:tt) => {{
        let n = $crate::spnl_arg!($n);
        $crate::spnl!(
            g $model (cross
                      (system "Your are an AI that combines prior outputs from other AIs, preferring no markdown or other exposition.")
                      $body
                      (user (format "Extract and simplify these {} final answers" n))))
    }};

    (combine $model:tt $body:tt) => (
        $crate::spnl!(
            g $model (cross
                      (system "Your are an AI that combines prior outputs from other AIs, preferring no markdown or other exposition.")
                      $body
                      (user "Combine and flatten these into one JSON array, preserving order")))
    );

    (cross $( $e:tt )+) => ( $crate::Unit::Cross(vec![$( $crate::spnl_arg!( $e ).into() ),+]) );
    (plus $e:tt) => ( $crate::Unit::Plus($crate::spnl_arg!( $e )) );
    (plus $( $e:tt )+) => ( $crate::Unit::Plus(vec![$( $crate::spnl_arg!( $e ).into() ),+]) );

    (repeat $n:tt $e:tt) => (spnl!(repeat i $n $e));
    (repeat $i:ident $n:tt $e:tt) => (spnl!(repeat $i 0 $n $e));
    (repeat $i:ident $start:tt $n:tt $e:tt) => {{
        let mut args: Vec<$crate::Unit> = vec![];
        let start = $crate::spnl_arg!($start);
        let end = $crate::spnl_arg!($n) + start;
        for $i in start..end {
            args.push($crate::spnl_arg!($e).clone());
        }
        args
    }};

    (g $model:tt $input:tt) => ($crate::spnl!(g $model $input 0.0 0));
    (g $model:tt $input:tt $temp:tt) => ($crate::spnl!(g $model $input $temp 0));
    (g $model:tt $input:tt $temp:tt $max_tokens:tt) => (
        $crate::Unit::Generate((
            $crate::spnl_arg!($model).to_string(),
            Box::new($crate::spnl_arg!($input).into()),
            $crate::spnl_arg!($max_tokens), $crate::spnl_arg!($temp)
        ))
    );

    (user $e:tt) => ($crate::Unit::User(($crate::spnl_arg!($e).clone().into(),)));
    (system $e:tt) => ($crate::Unit::System(($crate::spnl_arg!($e).into(),)));

    // execute rust
    //(rust $( $st:stmt )* ) => ( $($st);* );
    // other
    //($e:expr) => ($e.into());
}

#[macro_export]
macro_rules! spnl_arg {
    ( ( $($e:tt)* ) ) => ( $crate::spnl!( $($e)* ) );
    ($e:expr) => ($e);
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

    /// (model, input, max_tokens)
    #[serde(rename = "g")]
    Generate((String, Box<Unit>, i32, f32)),

    /// Loop
    Loop(Vec<Unit>),

    /// Ask with a given message
    Ask((String,)),
}
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
                Unit::Generate((m, _, _, _)) =>
                    style.paint(format!("\x1b[31;1mGenerate\x1b[0m \x1b[2m{m}\x1b[0m")),
                Unit::Repeat((n, _)) => style.paint(format!("Repeat {n}")),
                Unit::Loop(_) => style.paint("Loop".to_string()),
                Unit::Ask((m,)) => style.paint(format!("Ask {m}")),
                Unit::Print((m,)) => style.paint(format!("Print {}", truncate(m, 700))),
            }
        )
    }
    fn children(&self) -> ::std::borrow::Cow<[Self::Child]> {
        ::std::borrow::Cow::from(match self {
            Unit::Ask(_) | Unit::User(_) | Unit::System(_) | Unit::Print(_) => vec![],
            Unit::Plus(v) | Unit::Cross(v) => v.clone(),
            Unit::Loop(v) => v.clone(),
            Unit::Repeat((_, v)) => vec![*v.clone()],
            Unit::Generate((_, i, _, _)) => vec![*i.clone()],
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
pub fn from_str(s: &str) -> serde_lexpr::error::Result<Unit> {
    serde_lexpr::from_str(s)
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
    fn macro_user() {
        let result = spnl!(user "hello");
        assert_eq!(result, Unit::User(("hello".to_string(),)));
    }
    #[test]
    fn macro_system() {
        let result = spnl!(system "hello");
        assert_eq!(result, Unit::System(("hello".to_string(),)));
    }
    #[test]
    fn macro_ask() {
        let result = spnl!(ask "hello");
        assert_eq!(result, Unit::Ask(("hello".to_string(),)));
    }
    #[test]
    fn macro_plus_1() {
        let result = spnl!(plus (user "hello"));
        assert_eq!(result, Unit::Plus(vec![Unit::User(("hello".to_string(),))]));
    }
    #[test]
    fn macro_plus_2() {
        let result = spnl!(plus (user "hello") (system "world"));
        assert_eq!(
            result,
            Unit::Plus(vec![
                Unit::User(("hello".to_string(),)),
                Unit::System(("world".to_string(),))
            ])
        );
    }
    #[test]
    fn macro_cross_1() {
        let result = spnl!(cross (user "hello"));
        assert_eq!(
            result,
            Unit::Cross(vec![Unit::User(("hello".to_string(),))])
        );
    }
    #[test]
    fn macro_cross_3() {
        let result = spnl!(cross (user "hello") (system "world") (plus (user "sloop")));
        assert_eq!(
            result,
            Unit::Cross(vec![
                Unit::User(("hello".to_string(),)),
                Unit::System(("world".to_string(),)),
                Unit::Plus(vec![Unit::User(("sloop".to_string(),))])
            ])
        );
    }
    #[test]
    fn macro_gen() {
        let result = spnl!(g "ollama/granite3.2:2b" (user "hello") 0.0 0);
        assert_eq!(
            result,
            Unit::Generate((
                "ollama/granite3.2:2b".to_string(),
                Box::new(Unit::User(("hello".to_string(),))),
                0,
                0.0
            ))
        );
    }

    #[test]
    fn serde_user() -> Result<(), serde_lexpr::error::Error> {
        let result = from_str("(user \"hello\")")?;
        assert_eq!(result, Unit::User(("hello".to_string(),)));
        Ok(())
    }
    #[test]
    fn serde_system() -> Result<(), serde_lexpr::error::Error> {
        let result = from_str("(system \"hello\")")?;
        assert_eq!(result, Unit::System(("hello".to_string(),)));
        Ok(())
    }
    #[test]
    fn serde_ask() -> Result<(), serde_lexpr::error::Error> {
        let result = from_str("(ask \"hello\")")?;
        assert_eq!(result, Unit::Ask(("hello".to_string(),)));
        Ok(())
    }
    #[test]
    fn serde_plus_1() -> Result<(), serde_lexpr::error::Error> {
        let result = from_str("(plus (user \"hello\"))")?;
        assert_eq!(result, Unit::Plus(vec![Unit::User(("hello".to_string(),))]));
        Ok(())
    }
    #[test]
    fn serde_plus_2() -> Result<(), serde_lexpr::error::Error> {
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
    fn serde_cross_1() -> Result<(), serde_lexpr::error::Error> {
        let result = from_str("(cross (user \"hello\"))")?;
        assert_eq!(
            result,
            Unit::Cross(vec![Unit::User(("hello".to_string(),))])
        );
        Ok(())
    }
    #[test]
    fn serde_cross_3() -> Result<(), serde_lexpr::error::Error> {
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
    fn serde_gen() -> Result<(), serde_lexpr::error::Error> {
        let result = from_str("(g \"ollama/granite3.2:2b\" (user \"hello\") 0 0.0)")?;
        assert_eq!(
            result,
            Unit::Generate((
                "ollama/granite3.2:2b".to_string(),
                Box::new(Unit::User(("hello".to_string(),))),
                0,
                0.0
            ))
        );
        Ok(())
    }
}
