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
    #[arg(short, long, default_value_t = 0.0)]
    temperature: f32,
}
#[tokio::main]
async fn main() -> Result<(), SplError> {
    let args = Args::parse();
    let model = args.model;
    let temp = args.temperature;

    let program = spl!(
        g model
         (cross "Ask the model to select the best option from the candidates"
          (let
           ((max_tokens (ask "Max length of email?" 100))
            (prompt (format "write an introductory email for a job application, limited to at most {max_tokens} characters. use your imagination, go wild")))

           (plus "Generate candidate emails in parallel"
            (g model prompt max_tokens temp)
            (g model prompt max_tokens temp)
            (g model prompt max_tokens temp)
            (g model prompt max_tokens temp)
            (g model prompt max_tokens temp)
           )
          )

          "Compute an evaluation score that ranks each of the given candidate introductory emails, respond with a list such as [3,1,2,4] which ranks the emails from best to worst and uses their input index, and then print the best one and explain your thinking."

         )
    );

    let res = run(&program, None).await?;
    if res.to_string().len() > 0 {
        println!("{:?}", res);
    }
    Ok(())
}
