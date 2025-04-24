#[macro_export]
macro_rules! spl {
    // bool
    (false) => ($crate::Unit::Bool(false));
    (true) => ($crate::Unit::Bool(true));
    (self $(. $e:tt)* ) => (self $(. $e)* );

    // with-xxx
    /*(with-input-from-file ($var:ident $path:tt)
        $( ( $($e2:tt)* ) )*
    ) => ({
        use std;
        use std::io::Read;
        let $var = std::fs::File::open($crate::spl_arg!($path)).unwrap();
        $( $crate::spl!( $($e2)* ) );*
    });
    (with-input-from-mut-file ($var:ident $path:tt)
        $( ( $($e2:tt)* ) )*
    ) => ({
        use std;
        use std::io::Read;
        let mut $var = std::fs::File::open($crate::spl_arg!($path)).unwrap();
        $( $crate::spl!( $($e2)* ) );*
    });
    (with-output-to-new-file ($var:ident $path:tt)
        $( ( $($e2:tt)* ) )*
    ) => ({
        use std;
        use std::io::Write;
        let mut $var = std::fs::File::create($crate::spl_arg!($path)).unwrap();
        $( $crate::spl!( $($e2)* ) );*
    });*/

    //
    // for impl Read
    //
    /*(read $file:tt $s:ident) => ($crate::spl_arg!($file).read(&mut $crate::spl_arg!($s)));
    (read-to-string $file:tt $s:ident) => ($crate::spl_arg!($file).read_to_string(&mut $crate::spl_arg!($s)));
    (read-to-end $file:tt $s:ident) => ($crate::spl_arg!($file).read_to_end(&mut $crate::spl_arg!($s)));
    (read-exact $file:tt $s:ident) => ($crate::spl_arg!($file).read_exact(&mut $crate::spl_arg!($s)));
    (bytes $readable:tt) => ($crate::spl_arg!($readable).bytes());
    (chars $readable:tt) => ($crate::spl_arg!($readable).chars());
    (chain $readable:tt $next:tt) => ($crate::spl_arg!($readable).chain($next));
    (take $readable:tt $limit:tt) => ($crate::spl_arg!($readable).take($limit));


    //
    // for impl Write
    //
    (write $buffer:tt $e:tt) => ($crate::spl_arg!($buffer).write($crate::spl_arg!($e)));
    (write-all $buffer:tt $e:tt) => ($crate::spl_arg!($buffer).write_all($crate::spl_arg!($e)));
    (write-format $buffer:tt $fmt:tt) => ($crate::spl_arg!($buffer).write_fmt($crate::spl_arg!($fmt)));
    (flush $writable:tt) => ($crate::spl_arg!($writable).flush());*/

    // let
    (let ( $( ($var:ident $e:tt) )* )
        $( ( $($e2:tt)* ) )*
    ) => ({
        $(let mut $var = $crate::spl_arg!($e);)*
        $( $crate::spl!( $($e2)* ) );*
    });

    // with-xxx
    /*(with-input-from-file ($var:ident $path:tt)
        $( ( $($e2:tt)* ) )*
    ) => ({
        use std;
        use std::io::Read;
        let $var = std::fs::File::open($crate::spl_arg!($path)).unwrap();
        $( $crate::spl!( $($e2)* ) );*
    });
    (with-input-from-mut-file ($var:ident $path:tt)
        $( ( $($e2:tt)* ) )*
    ) => ({
        use std;
        use std::io::Read;
        let mut $var = std::fs::File::open($crate::spl_arg!($path)).unwrap();
        $( $crate::spl!( $($e2)* ) );*
    });

    (with-output-to-new-file ($var:ident $path:tt)
        $( ( $($e2:tt)* ) )*
    ) => ({
        use std;
        use std::io::Write;
        let mut $var = std::fs::File::create($crate::spl_arg!($path)).unwrap();
        $( $crate::spl!( $($e2)* ) );*
    });*/

    // loop
    (loop $( ( $($e:tt)* ) )* ) => ( loop { $( $crate::spl!( $($e)* ) );* });

    // dotimes
    (dotimes ($var:ident $count:tt) $( ( $($e:tt)* ) )* ) => (
        for $var in 0..$crate::lisp_arg!($count) {
            $( $crate::lisp!( $($e)* ) );*
        }
    );

    // if
    (if ( $($cond:tt)* ) $e1:tt $e2:tt) => (if $crate::spl!($($cond)*) { $crate::spl!($e1) }else{ $crate::spl!($e2) });
    (if ( $($cond:tt)* ) $e:tt) => (if $crate::spl!($($cond)*) { $crate::spl!($e) });
    (if $cond:tt $e1:tt $e2:tt) => (if $cond { $crate::spl!($e1) }else{ $crate::spl!($e2) });
    (if $cond:tt $e:tt) => (if $cond { $crate::spl!($e) });

    // compare
    (eq $x:tt $y:tt) => ($crate::spl_arg!($x) == $crate::spl_arg!($y));
    (== $x:tt $y:tt) => ($crate::spl_arg!($x) == $crate::spl_arg!($y));
    (!= $x:tt $y:tt) => ($crate::spl_arg!($x) != $crate::spl_arg!($y));
    (< $x:tt $y:tt) => ($crate::spl_arg!($x) < $crate::spl_arg!($y));
    (> $x:tt $y:tt) => ($crate::spl_arg!($x) > $crate::spl_arg!($y));
    (<= $x:tt $y:tt) => ($crate::spl_arg!($x) <= $crate::spl_arg!($y));
    (>= $x:tt $y:tt) => ($crate::spl_arg!($x) >= $crate::spl_arg!($y));

    (print $( $e:tt )+) => ( print!( $($e),+ ) );
    (println $( $e:tt )+) => ( println!( $($e),+ ) );

    // math
    (+ $x:tt $y:tt) => ($crate::spl_arg!($x) + $crate::spl_arg!($y));
    (- $x:tt $y:tt) => ($crate::spl_arg!($x) - $crate::spl_arg!($y));
    (* $x:tt $y:tt) => ($crate::spl_arg!($x) * $crate::spl_arg!($y));
    (/ $x:tt $y:tt) => ($crate::spl_arg!($x) / $crate::spl_arg!($y));
    (% $x:tt $y:tt) => ($crate::spl_arg!($x) % $crate::spl_arg!($y));

    (file $f:tt) => ($crate::Unit::String(::std::fs::read_to_string($crate::spl_arg!($f)).unwrap()));
    (cross $( $e:tt )+) => {{
        let mut args: Vec<$crate::Unit> = vec![];
        $(
            args.push($crate::spl_arg!($e).into());
        )+
        $crate::Unit::Cross(args)
    }};
    (plus $( $e:tt )+) => {{
        let mut args: Vec<$crate::Unit> = vec![];
        $(
            args.push($crate::spl_arg!($e).into());
        )+
        $crate::Unit::Plus(args)
    }};

    (g $model:tt $input:tt) => ($crate::Unit::Generate(($crate::spl_arg!($model), Box::new($crate::spl_arg!($input).into()))));
}

#[macro_export]
macro_rules! spl_arg {
    ( ( $($e:tt)* ) ) => ( $crate::spl!( $($e)* ) );
    ($e:expr) => ($e);
}

#[derive(Debug, Clone)]
pub enum Unit<'a> {
    Bool(bool),
    Number(usize),
    Slice(&'a str),
    String(String),
    Cross(Vec<Unit<'a>>),
    Plus(Vec<Unit<'a>>),

    /// (model, input)
    Generate((&'a str, Box<Unit<'a>>)),
}
impl<'a> From<&'a str> for Unit<'a> {
    fn from(s: &'a str) -> Self {
        Self::Slice(s)
    }
}
impl<'a> ::std::fmt::Display for Unit<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Unit::Cross(v) | Unit::Plus(v) => write!(f, "{:?}", v),
            Unit::Bool(b) => write!(f, "{}", b),
            Unit::Number(n) => write!(f, "{}", n),
            Unit::Slice(s) => write!(f, "{}", s),
            Unit::String(s) => write!(f, "{}", s),
            Unit::Generate((model, input)) => write!(f, "model={} input={:?}", model, input),
        }
    }
}
impl<'a> PartialEq for Unit<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Unit::Number(a), Unit::Number(b)) => a == b,
            (Unit::Slice(a), Unit::Slice(b)) => a == b,
            (Unit::String(a), Unit::String(b)) => a == b,
            (Unit::Bool(a), Unit::Bool(b)) => a == b,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = spl!(true);
        assert_eq!(result, Unit::Bool(true));
    }
}
