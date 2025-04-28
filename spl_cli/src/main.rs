use spl_ast::spl;
use spl_run::{result::SplError, run};

#[tokio::main]
async fn main() -> Result<(), SplError> {
    let program = spl!(
        let (//(model "ollama/granite3.2:2b"))
            (model "openai/ibm-granite/granite-3.3-8b-instruct"))
            (g model
             (cross "Ask the model to select the best option from the candidates"
              (let
               ((max_tokens (ask "Max length of email?" 100))
                (temp (ask "Temperature?" 0.5))
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
            )
    );

    let res = run(&program, None).await?;
    if res.to_string().len() > 0 {
        println!("{:?}", res);
    }
    Ok(())
}
