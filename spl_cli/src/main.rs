use spl_ast::spl;
use spl_run::{result::SplError, run};

use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Model
    #[arg(short, long, default_value = "ollama/granite3.2:2b")]
    model: String,

    /// Temperature
    #[arg(short, long, default_value_t = 0.5)]
    temperature: f32,

    /// Max Completion/Generated Tokens
    #[arg(short = 'l', long, default_value_t = 100)]
    max_tokens: i32,

    /// Number of candidate emails to consider
    #[arg(short, long, default_value_t = 5)]
    n: u32,
}
#[tokio::main]
async fn main() -> Result<(), SplError> {
    let args = Args::parse();
    let model = args.model;
    let temp = args.temperature;
    let max_tokens = args.max_tokens;
    let n = args.n;

    let program = spl!(
        g model
         (cross "Ask the model to select the best option from the candidates"
          (system (format "You compute an evaluation score that ranks {n} given candidate introductory emails. Better emails are ones that mention specifics, such as names of people and companies. You always explain your thinking by presenting a list of the top 3 ordered by their rank, and finish by showing me the best one."))

          (plusn n (format "Generate {n} candidate emails in parallel")
           (g model
            (format "write an introductory email for a job application, limited to at most {max_tokens} characters. use your imagination, go wild.")
            max_tokens temp)
          )

          "My name is Shiloh. I am a data scientist with 10 years of experience and need an introductory email to apply for a position at IBM in their research department"
         )
    );

    let res = run(&program, None).await?;
    if res.to_string().len() > 0 {
        println!("{:?}", res);
    }
    Ok(())
}
