use spl_ast::spl;
use spl_run::{result::SplError, run};

#[tokio::main]
async fn main() -> Result<(), SplError> {
    //let program = spl!(g "ollama/granite3.2:2b" (cross "Sample cross" (file "/tmp/foo") "Tell me a story" (ask "What is your question")));
    let program = spl!(
        let ((model "ollama/granite3.2:2b"))
            (g model
             (cross "Ask the model to select the best option from the candidates"
              "Compute an evaluation score that ranks each of the given candidate introductory emails, respond with a list such as [3,1,2,4] which ranks the emails from best to worst and uses their input index, and then print the best one."

              (let
               ((max_tokens (askn "Max length of email?"))
                (temp (askf "Temperature?"))
                (prompt (format "write an introductory email, limited to at most {max_tokens} characters")))

               (plus "Generate candidate emails in parallel"
                (g model prompt max_tokens temp)
                (g model prompt max_tokens temp)
                (g model prompt max_tokens temp)
                (g model prompt max_tokens temp)
                (g model prompt max_tokens temp)
               )
              )
             )
            )
    );

    let res = run(&program, None).await?;
    if res.to_string().len() > 0 {
        println!("{:?}", res);
    }
    Ok(())
}
