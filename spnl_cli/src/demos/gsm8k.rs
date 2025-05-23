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

    let chunks = spnl!(chunk n chunk_size "Question " "./gsm8k-questions.json")
        .map(|chunk| {
            spnl!(
                extract model chunk_size
                    (g model
                     (cross
                      (system "You are an AI that reasons about math word problems")
                      (plus chunk)
                     ))
            )
        })
        .collect();

    Ok(spnl!(combine model (plus chunks)))
}
