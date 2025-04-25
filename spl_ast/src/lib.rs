#[macro_export]
macro_rules! spl {
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

    // read as string from stdin
    (ask $message:tt) => {{
        println!("{}", $crate::spl_arg!($message));
        let mut buffer = String::new();
        let mut bytes_read = ::std::io::stdin().read_line(&mut buffer)?;
        buffer
    }};

    // read as i32 from stdin
    (askn $message:tt) => ($crate::spl!(askd $message 100));
    // read as f32 from stdin
    (askf $message:tt) => ($crate::spl!(askd $message 0.5));

    // read with default value
    (askd $message:tt $default:tt) => {{
        let default = $crate::spl_arg!($default);
        println!("{} [default={default}]", $crate::spl_arg!($message));
        let mut buffer = String::new();
        let mut bytes_read = ::std::io::stdin().read_line(&mut buffer)?;
        if buffer.trim().len() == 0 {
            default
        } else {
            buffer.trim().parse()?
        }
    }};

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
    (format $( $e:tt )+) => ( &format!( $($e),+ ) );

    // math
    (+ $x:tt $y:tt) => ($crate::spl_arg!($x) + $crate::spl_arg!($y));
    (- $x:tt $y:tt) => ($crate::spl_arg!($x) - $crate::spl_arg!($y));
    (* $x:tt $y:tt) => ($crate::spl_arg!($x) * $crate::spl_arg!($y));
    (/ $x:tt $y:tt) => ($crate::spl_arg!($x) / $crate::spl_arg!($y));
    (% $x:tt $y:tt) => ($crate::spl_arg!($x) % $crate::spl_arg!($y));

    (file $f:tt) => ($crate::Unit::String(::std::fs::read_to_string($crate::spl_arg!($f)).unwrap()));
    (cross $description:tt $( $e:tt )+) => {{
        let mut args: Vec<$crate::Unit> = vec![];
        $(
            args.push($crate::spl_arg!($e).into());
        )+
        $crate::Unit::Cross(($crate::spl_arg!($description).into(), args))
    }};
    (plus $description:tt $( $e:tt )+) => {{
        let mut args: Vec<$crate::Unit> = vec![];
        $(
            args.push($crate::spl_arg!($e).into());
        )+
        $crate::Unit::Plus(($crate::spl_arg!($description).into(), args))
    }};

    (g $model:tt $input:tt) => ($crate::spl!(g $model $input 0 0.0));
    (g $model:tt $input:tt $max_tokens:tt) => ($crate::spl!(g $model $input $max_tokens 0.0));
    (g $model:tt $input:tt $max_tokens:tt $temp:tt) => (
        $crate::Unit::Generate((
            $crate::spl_arg!($model).to_string(),
            Box::new($crate::spl_arg!($input).into()),
            $crate::spl_arg!($max_tokens), $crate::spl_arg!($temp)
        ))
    );

    // Other
    ($e:expr) => (Unit::String($e.into()));
}

#[macro_export]
macro_rules! spl_arg {
    ( ( $($e:tt)* ) ) => ( $crate::spl!( $($e)* ) );
    ($e:expr) => ($e);
}

#[derive(Debug, Clone)]
pub enum Unit {
    String(String),

    /// (description, units)
    Cross((String, Vec<Unit>)),

    /// (description, units)
    Plus((String, Vec<Unit>)),

    /// (model, input, max_tokens)
    Generate((String, Box<Unit>, i32, f32)),
}
impl ::std::fmt::Display for Unit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Unit::Cross((d, v)) | Unit::Plus((d, v)) => write!(f, "{}: {:?}", d, v),
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
        let result = spl!("hello");
        assert_eq!(result, Unit::String("hello".to_string()));
    }
}
