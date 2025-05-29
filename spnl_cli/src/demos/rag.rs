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
        embedding_model,
        n,
        ..
    } = args;

    Ok(spnl!(g model
             (with embedding_model
              (user "Does PDL have a contribute keyword?")
              (file "./rag-doc1.json"))))
}
