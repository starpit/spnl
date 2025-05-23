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

    let combine_system_prompt = r#"Your are an AI that combines prior outputs from other AIs."#;
    let solve_system_prompt = r#"Your are an AI that reasons about math word problems."#;

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
        .map(|chunk| {
            spnl!(
            g model
                (cross
                 (system combine_system_prompt)
                 (g model (cross (system solve_system_prompt) (plus chunk)))
                 (user (format "Extract and simplify the {chunk_size} final answers"))))
        })
        .collect();

    Ok(spnl!(g model
          (cross
           (system combine_system_prompt)
           (plus chunks)
           (user "Combine and flatten these into one JSON array, preserving order")
          )
    ))
}

//  by responding with a plain JSON array of strings or numbers such as ["a","b","c"] or [5,"y","9m"] or ["hello","world"], no markdown or html or any other extra text
//(user (format "Extract the {chunk_size} final answers into a JSON array with just the answers")))))
//(user (format r#"Extract the {n} final answers into a JSON array with JSON entries "{{"question": 3, "answer": "simplified numerical answer"}}""#)))))
//(user (format "Extract each of the {n} final answers ainto a JSON array with just the answers, preserving order")))))
//(user (format "Extract these into one JSON array")))))
