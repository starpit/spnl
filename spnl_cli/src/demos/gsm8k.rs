use crate::args::Args;
use spnl::{Unit, spnl};

#[derive(serde::Deserialize)]
struct Problem {
    question: String,
    // answer: String,
}

pub fn demo(args: Args) -> Result<Unit, Box<dyn ::std::error::Error>> {
    let Args {
        model,
        n,
        chunk_size,
        ..
    } = args;

    Ok(spnl!(combine model
             (plus (chunk chunk_size
                    (prefix "Question " (take n (file "./gsm8k-questions.json")))
                    (lambda (parts)
                     (extract model chunk_size
                      (g model
                       (cross
                        (system "You are an AI that reasons about math word problems")
                        (plus parts)))))))
    ))
}
