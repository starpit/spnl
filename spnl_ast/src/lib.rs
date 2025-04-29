// pub use dialoguer::Input;

// Inspiration: https://github.com/JunSuzukiJapan/macro-lisp
#[macro_export]
macro_rules! spnl {
    // bool
    //(false) => ($crate::Unit::Bool(false));
    //(true) => ($crate::Unit::Bool(true));
    //(self $(. $e:tt)* ) => (self $(. $e)* );

    // with-xxx
    /*(with-input-from-file ($var:ident $path:tt)
        $( ( $($e2:tt)* ) )*
    ) => ({
        use std;
        use std::io::Read;
        let $var = std::fs::File::open($crate::spnl_arg!($path)).unwrap();
        $( $crate::spnl!( $($e2)* ) );*
    });
    (with-input-from-mut-file ($var:ident $path:tt)
        $( ( $($e2:tt)* ) )*
    ) => ({
        use std;
        use std::io::Read;
        let mut $var = std::fs::File::open($crate::spnl_arg!($path)).unwrap();
        $( $crate::spnl!( $($e2)* ) );*
    });
    (with-output-to-new-file ($var:ident $path:tt)
        $( ( $($e2:tt)* ) )*
    ) => ({
        use std;
        use std::io::Write;
        let mut $var = std::fs::File::create($crate::spnl_arg!($path)).unwrap();
        $( $crate::spnl!( $($e2)* ) );*
    });*/

    //
    // for impl Read
    //
    /*(read $file:tt $s:ident) => ($crate::spnl_arg!($file).read(&mut $crate::spnl_arg!($s)));
    (read-to-string $file:tt $s:ident) => ($crate::spnl_arg!($file).read_to_string(&mut $crate::spnl_arg!($s)));
    (read-to-end $file:tt $s:ident) => ($crate::spnl_arg!($file).read_to_end(&mut $crate::spnl_arg!($s)));
    (read-exact $file:tt $s:ident) => ($crate::spnl_arg!($file).read_exact(&mut $crate::spnl_arg!($s)));
    (bytes $readable:tt) => ($crate::spnl_arg!($readable).bytes());
    (chars $readable:tt) => ($crate::spnl_arg!($readable).chars());
    (chain $readable:tt $next:tt) => ($crate::spnl_arg!($readable).chain($next));
    (take $readable:tt $limit:tt) => ($crate::spnl_arg!($readable).take($limit));


    //
    // for impl Write
    //
    (write $buffer:tt $e:tt) => ($crate::spnl_arg!($buffer).write($crate::spnl_arg!($e)));
    (write-all $buffer:tt $e:tt) => ($crate::spnl_arg!($buffer).write_all($crate::spnl_arg!($e)));
    (write-format $buffer:tt $fmt:tt) => ($crate::spnl_arg!($buffer).write_fmt($crate::spnl_arg!($fmt)));
    (flush $writable:tt) => ($crate::spnl_arg!($writable).flush());*/

    // let
    (let ( $( ($var:ident $e:tt) )* )
        $( ( $($e2:tt)* ) )*
    ) => ({
        $(let mut $var = $crate::spnl_arg!($e);)*
        $( $crate::spnl!( $($e2)* ) );*
    });

    // with-xxx
    /*(with-input-from-file ($var:ident $path:tt)
        $( ( $($e2:tt)* ) )*
    ) => ({
        use std;
        use std::io::Read;
        let $var = std::fs::File::open($crate::spnl_arg!($path)).unwrap();
        $( $crate::spnl!( $($e2)* ) );*
    });
    (with-input-from-mut-file ($var:ident $path:tt)
        $( ( $($e2:tt)* ) )*
    ) => ({
        use std;
        use std::io::Read;
        let mut $var = std::fs::File::open($crate::spnl_arg!($path)).unwrap();
        $( $crate::spnl!( $($e2)* ) );*
    });

    (with-output-to-new-file ($var:ident $path:tt)
        $( ( $($e2:tt)* ) )*
    ) => ({
        use std;
        use std::io::Write;
        let mut $var = std::fs::File::create($crate::spnl_arg!($path)).unwrap();
        $( $crate::spnl!( $($e2)* ) );*
    });*/

    // read as string from stdin
    (ask $message:tt) => ( Unit::Ask(($crate::spnl_arg!($message).into(), None)) );

    // read with default value
    (ask $message:tt $default:tt) => ( Unit::Ask(($crate::spnl_arg!($message).into(), Some($crate::spnl_arg!($default).into()), None)) );

    // loop
    // (loop $( ( $($e:tt)* ) )* ) => ( loop { $( $crate::spnl!( $($e)* ) );* } );
    (loop $( ( $($e:tt)* ) )* ) => ( $crate::Unit::Loop(vec![$( $crate::spnl!( $($e)* ) ),*]) );

    // dotimes
    (dotimes ($var:ident $count:tt) $( ( $($e:tt)* ) )* ) => (
        for $var in 0..$crate::spnl_arg!($count) {
            $( $crate::spnl!( $($e)* ) );*
        }
    );

    // if
    (if ( $($cond:tt)* ) $e1:tt $e2:tt) => (if $crate::spnl!($($cond)*) { $crate::spnl!($e1) }else{ $crate::spnl!($e2) });
    (if ( $($cond:tt)* ) $e:tt) => (if $crate::spnl!($($cond)*) { $crate::spnl!($e) });
    (if $cond:tt $e1:tt $e2:tt) => (if $cond { $crate::spnl!($e1) }else{ $crate::spnl!($e2) });
    (if $cond:tt $e:tt) => (if $cond { $crate::spnl!($e) });

    // compare
    (eq $x:tt $y:tt) => ($crate::spnl_arg!($x) == $crate::spnl_arg!($y));
    (== $x:tt $y:tt) => ($crate::spnl_arg!($x) == $crate::spnl_arg!($y));
    (!= $x:tt $y:tt) => ($crate::spnl_arg!($x) != $crate::spnl_arg!($y));
    (< $x:tt $y:tt) => ($crate::spnl_arg!($x) < $crate::spnl_arg!($y));
    (> $x:tt $y:tt) => ($crate::spnl_arg!($x) > $crate::spnl_arg!($y));
    (<= $x:tt $y:tt) => ($crate::spnl_arg!($x) <= $crate::spnl_arg!($y));
    (>= $x:tt $y:tt) => ($crate::spnl_arg!($x) >= $crate::spnl_arg!($y));

    (print $( $e:tt )+) => ( print!( $($e),+ ) );
    (println $( $e:tt )+) => ( println!( $($e),+ ) );
    (format $( $e:tt )+) => ( &format!( $($e),+ ) );

    // math
    (+ $x:tt $y:tt) => ($crate::spnl_arg!($x) + $crate::spnl_arg!($y));
    (- $x:tt $y:tt) => ($crate::spnl_arg!($x) - $crate::spnl_arg!($y));
    (* $x:tt $y:tt) => ($crate::spnl_arg!($x) * $crate::spnl_arg!($y));
    (/ $x:tt $y:tt) => ($crate::spnl_arg!($x) / $crate::spnl_arg!($y));
    (% $x:tt $y:tt) => ($crate::spnl_arg!($x) % $crate::spnl_arg!($y));

    (file $f:tt) => ($crate::Unit::String(::std::fs::read_to_string($crate::spnl_arg!($f)).unwrap()));
    (cross $description:tt $( $e:tt )+) => {{
        let mut args: Vec<$crate::Unit> = vec![];
        $(
            args.push($crate::spnl_arg!($e).into());
        )+
        $crate::Unit::Cross((Some($crate::spnl_arg!($description).into()), args))
    }};
    (cross $description:tt $( $e:tt )+) => {{
        let mut args: Vec<$crate::Unit> = vec![];
        $(
            args.push($crate::spnl_arg!($e).into());
        )+
        $crate::Unit::Cross((Some($crate::spnl_arg!($description).into()), args))
    }};
    (plus $description:tt $( $e:tt )+) => {{
        let mut args: Vec<$crate::Unit> = vec![];
        $(
            args.push($crate::spnl_arg!($e).into());
        )+
        $crate::Unit::Plus((Some($crate::spnl_arg!($description).into()), args))
    }};
    (plusn $n:tt $description:tt $e:tt) => {{
        let mut args: Vec<$crate::Unit> = vec![];
        for i in 0..$crate::spnl_arg!($n) {
            args.push($crate::spnl_arg!($e).into());
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

    (system $e:tt) => ($crate::Unit::System($crate::spnl_arg!($e).into()));

    // execute rust
    (rust $( $st:stmt )* ) => ( $($st);* );
    // other
    ($e:expr) => (Unit::String($e.into()));
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

    /// Ask (prompt, default)
    Ask((String, Option<String>)),
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
