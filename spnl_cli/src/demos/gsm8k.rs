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

    let problems = serde_json::Deserializer::from_str(include_str!("./gsm8k.jsonl"))
        .into_iter::<Problem>()
        .take(n as usize)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .enumerate()
        .map(|(idx, p)| (1 + (idx % chunk_size), p))
        .map(|(idx, Problem { question, .. })| spnl!(user (format "Question {idx}: {question}")))
        .collect::<Vec<_>>();

    let chunks = problems
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .map(|chunk| spnl!(
            extract model chunk_size
                (g model (cross (system "Your are an AI that reasons about math word problems") (plus chunk)))
        ))
        .collect();

    Ok(spnl!(combine model chunks))
}
