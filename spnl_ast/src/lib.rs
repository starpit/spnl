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
    (ask $message:tt) => ( $crate::Unit::Ask($crate::spnl_arg!($message).into()) );

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
    (format $( $e:tt )+) => ( &format!( $($e),+ ) );

    // math
    /*(+ $x:tt $y:tt) => ($crate::spnl_arg!($x) + $crate::spnl_arg!($y));*/
    (- $x:tt $y:tt) => ($crate::spnl_arg!($x) - $crate::spnl_arg!($y));
    /*(* $x:tt $y:tt) => ($crate::spnl_arg!($x) * $crate::spnl_arg!($y));
    (/ $x:tt $y:tt) => ($crate::spnl_arg!($x) / $crate::spnl_arg!($y));
    (% $x:tt $y:tt) => ($crate::spnl_arg!($x) % $crate::spnl_arg!($y));*/

    (file $f:tt) => (include_str!($crate::spnl_arg!($f)));
    (cross (desc $description:tt) $( $e:tt )+) => (
        $crate::Unit::Cross((Some($crate::spnl_arg!($description).into()), vec![$( $crate::spnl_arg!( $e ).into() ),+]))
    );
    (cross $( $e:tt )+) => ( $crate::Unit::Cross((None, vec![$( $crate::spnl_arg!( $e ).into() ),+])) );
    (plus (desc $description:tt) $( $e:tt )+) => (
        $crate::Unit::Plus((Some($crate::spnl_arg!($description).into()), vec![$( $crate::spnl_arg!( $e ).into() ),+]))
    );
    (plus $( $e:tt )+) => ( $crate::Unit::Plus((None, vec![$( $crate::spnl_arg!( $e ).into() ),+])) );
    (plusn $n:tt $description:tt $e:tt) => {{
        let mut args: Vec<$crate::Unit> = vec![];
        for i in 0..$crate::spnl_arg!($n) {
            args.push($crate::spnl_arg!($e).clone());
        }
        $crate::Unit::Plus((Some($crate::spnl_arg!($description).into()), args))
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

    (user $e:tt) => ($crate::Unit::String($e.clone().into()));
    (system $e:tt) => ($crate::Unit::System($crate::spnl_arg!($e).into()));

    // execute rust
    //(rust $( $st:stmt )* ) => ( $($st);* );
    // other
    ($e:expr) => ($crate::Unit::String($e.into()));
}

#[macro_export]
macro_rules! spnl_arg {
    ( ( $($e:tt)* ) ) => ( $crate::spnl!( $($e)* ) );
    ($e:expr) => ($e);
}

#[derive(Debug, Clone)]
pub enum Unit {
    String(String),

    /// System prompt
    System(String),

    /// (description, units)
    Cross((Option<String>, Vec<Unit>)),

    /// (description, units)
    Plus((Option<String>, Vec<Unit>)),

    /// (model, input, max_tokens)
    Generate((String, Box<Unit>, i32, f32)),

    /// Loop
    Loop(Vec<Unit>),

    /// Ask with a given message>
    Ask(String),
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
                Unit::String(s) => style.paint(format!("\x1b[33mUser\x1b[0m {}", truncate(s, 70))),
                Unit::System(s) =>
                    style.paint(format!("\x1b[34mSystem\x1b[0m {}", truncate(s, 70))),
                Unit::Plus((d, _)) => style.paint(format!(
                    "\x1b[31;1mPlus\x1b[0m {}",
                    d.as_deref().unwrap_or("")
                )),
                Unit::Cross((d, _)) => style.paint(format!(
                    "\x1b[31;1mCross\x1b[0m {}",
                    d.as_deref().unwrap_or("")
                )),
                Unit::Generate((m, _, _, _)) =>
                    style.paint(format!("\x1b[31;1mGenerate\x1b[0m \x1b[2m{m}\x1b[0m")),
                Unit::Loop(_) => style.paint("Loop".to_string()),
                Unit::Ask(m) => style.paint(format!("Ask {m}")),
            }
        )
    }
    fn children(&self) -> ::std::borrow::Cow<[Self::Child]> {
        ::std::borrow::Cow::from(match self {
            Unit::Ask(_) | Unit::String(_) | Unit::System(_) => vec![],
            Unit::Plus((_, v)) | Unit::Cross((_, v)) => v.clone(),
            Unit::Loop(v) => v.clone(),
            Unit::Generate((_, i, _, _)) => vec![*i.clone()],
        })
    }
}
impl ::std::fmt::Display for Unit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Unit::Cross((d, v)) | Unit::Plus((d, v)) => {
                if let Some(description) = d {
                    write!(f, "{}: {:?}", description, v)
                } else {
                    write!(f, "{:?}", v)
                }
            }
            Unit::String(s) => write!(f, "{}", s),
            _ => Ok(()),
        }
    }
}
impl From<&str> for Unit {
    fn from(s: &str) -> Self {
        Self::String(s.into())
    }
}
impl ::std::str::FromStr for Unit {
    type Err = Box<dyn ::std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::String(s.to_string()))
    }
}
impl From<&String> for Unit {
    fn from(s: &String) -> Self {
        Self::String(s.clone())
    }
}
impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Unit::String(a), Unit::String(b)) => a == b,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = spnl!("hello");
        assert_eq!(result, Unit::String("hello".to_string()));
    }
}
